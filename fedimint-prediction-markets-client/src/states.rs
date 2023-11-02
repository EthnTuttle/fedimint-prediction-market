use fedimint_client::sm::{DynState, OperationId, State, StateTransition};
use fedimint_client::transaction::TxSubmissionError;
use fedimint_client::DynGlobalClientContext;
use fedimint_core::core::{IntoDynInstance, ModuleInstanceId};
use fedimint_core::encoding::{Decodable, Encodable};
use fedimint_core::TransactionId;
use fedimint_prediction_markets_common::OrderIdClientSide;
// use serde::{Deserialize, Serialize};
// use thiserror::Error;

use crate::{PredictionMarketsClientContext, PredictionMarketsClientModule};

/// Tracks a transaction. Not being used currently for prediction markets
#[derive(Debug, Clone, Eq, PartialEq, Decodable, Encodable)]
pub enum PredictionMarketsStateMachine {
    NewMarket {
        operation_id: OperationId,
        tx_id: TransactionId,
    },
    NewMarketAccepted(OperationId),
    NewMarketFailed(OperationId),

    PayoutMarket {
        operation_id: OperationId,
        tx_id: TransactionId,
    },
    PayoutMarketAccepted(OperationId),
    PayoutMarketFailed(OperationId),

    NewOrder {
        operation_id: OperationId,
        tx_id: TransactionId,
        order: OrderIdClientSide,
        sources: Vec<OrderIdClientSide>,
    },
    NewOrderAccepted(OperationId),
    NewOrderFailed(OperationId),

    CancelOrder {
        operation_id: OperationId,
        tx_id: TransactionId,
        order: OrderIdClientSide,
    },
    CancelOrderAccepted(OperationId),
    CancelOrderFailed(OperationId),
}

impl State for PredictionMarketsStateMachine {
    type ModuleContext = PredictionMarketsClientContext;
    type GlobalContext = DynGlobalClientContext;

    fn transitions(
        &self,
        _context: &Self::ModuleContext,
        global_context: &Self::GlobalContext,
    ) -> Vec<StateTransition<Self>> {
        match self.clone() {
            Self::NewMarket {
                operation_id,
                tx_id,
            } => {
                vec![StateTransition::new(
                    await_tx_accepted(global_context.clone(), operation_id, tx_id),
                    move |_dbtx, res, _state: Self| match res {
                        // tx accepted
                        Ok(_) => Box::pin(async move {
                            PredictionMarketsStateMachine::NewMarketAccepted(operation_id)
                        }),
                        // tx rejected
                        Err(_) => Box::pin(async move {
                            PredictionMarketsStateMachine::NewMarketFailed(operation_id)
                        }),
                    },
                )]
            }
            Self::NewMarketAccepted(_) => vec![],
            Self::NewMarketFailed(_) => vec![],

            Self::PayoutMarket {
                operation_id,
                tx_id,
            } => {
                vec![StateTransition::new(
                    await_tx_accepted(global_context.clone(), operation_id, tx_id),
                    move |_dbtx, res, _state: Self| match res {
                        // tx accepted
                        Ok(_) => Box::pin(async move {
                            PredictionMarketsStateMachine::PayoutMarketAccepted(operation_id)
                        }),
                        // tx rejected
                        Err(_) => Box::pin(async move {
                            PredictionMarketsStateMachine::PayoutMarketFailed(operation_id)
                        }),
                    },
                )]
            }
            Self::PayoutMarketAccepted(_) => vec![],
            Self::PayoutMarketFailed(_) => vec![],

            Self::NewOrder {
                operation_id,
                tx_id,
                order,
                sources,
            } => {
                vec![StateTransition::new(
                    await_tx_accepted(global_context.clone(), operation_id, tx_id),
                    move |dbtx, res, _state: Self| match res {
                        // tx accepted
                        Ok(_) => {
                            let sources = sources.clone();
                            Box::pin(async move {
                                PredictionMarketsClientModule::new_order_accepted(
                                    dbtx.module_tx(),
                                    order,
                                    sources,
                                )
                                .await;
                                PredictionMarketsStateMachine::NewOrderAccepted(operation_id)
                            })
                        }
                        // tx rejected
                        Err(_) => Box::pin(async move {
                            PredictionMarketsClientModule::new_order_failed(
                                dbtx.module_tx(),
                                order,
                            )
                            .await;
                            PredictionMarketsStateMachine::NewOrderFailed(operation_id)
                        }),
                    },
                )]
            }
            Self::NewOrderAccepted(_) => vec![],
            Self::NewOrderFailed(_) => vec![],

            Self::CancelOrder {
                operation_id,
                tx_id,
                order,
            } => {
                vec![StateTransition::new(
                    await_tx_accepted(global_context.clone(), operation_id, tx_id),
                    move |dbtx, res, _state: Self| match res {
                        // tx accepted
                        Ok(_) => Box::pin(async move {
                            PredictionMarketsClientModule::cancel_order_accepted(
                                dbtx.module_tx(),
                                order,
                            )
                            .await;
                            PredictionMarketsStateMachine::CancelOrderAccepted(operation_id)
                        }),
                        // tx rejected
                        Err(_) => Box::pin(async move {
                            PredictionMarketsStateMachine::CancelOrderFailed(operation_id)
                        }),
                    },
                )]
            }
            Self::CancelOrderAccepted(_) => vec![],
            Self::CancelOrderFailed(_) => vec![],
        }
    }

    fn operation_id(&self) -> OperationId {
        match self {
            Self::NewMarket {
                operation_id,
                tx_id: _,
            } => *operation_id,
            Self::NewMarketAccepted(operation_id) => *operation_id,
            Self::NewMarketFailed(operation_id) => *operation_id,

            Self::PayoutMarket {
                operation_id,
                tx_id: _,
            } => *operation_id,
            Self::PayoutMarketAccepted(operation_id) => *operation_id,
            Self::PayoutMarketFailed(operation_id) => *operation_id,

            Self::NewOrder {
                operation_id,
                tx_id: _,
                order: _,
                sources: _,
            } => *operation_id,
            Self::NewOrderAccepted(operation_id) => *operation_id,
            Self::NewOrderFailed(operation_id) => *operation_id,

            Self::CancelOrder {
                operation_id,
                tx_id: _,
                order: _,
            } => *operation_id,
            Self::CancelOrderAccepted(operation_id) => *operation_id,
            Self::CancelOrderFailed(operation_id) => *operation_id,
        }
    }
}

// TODO: Boiler-plate, should return OutputOutcome
async fn await_tx_accepted(
    context: DynGlobalClientContext,
    id: OperationId,
    txid: TransactionId,
) -> Result<(), TxSubmissionError> {
    context.await_tx_accepted(id, txid).await
}

// TODO: Boiler-plate
impl IntoDynInstance for PredictionMarketsStateMachine {
    type DynType = DynState<DynGlobalClientContext>;

    fn into_dyn(self, instance_id: ModuleInstanceId) -> Self::DynType {
        DynState::from_typed(instance_id, self)
    }
}

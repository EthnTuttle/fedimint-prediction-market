use std::collections::BTreeMap;

use fedimint_core::api::{FederationApiExt, FederationResult, IModuleFederationApi};

use fedimint_core::module::ApiRequestErased;
use fedimint_core::task::{MaybeSend, MaybeSync};
use fedimint_core::{apply, async_trait_maybe_send, Amount, OutPoint};
use fedimint_prediction_markets_common::api::{
    WaitMarketOutcomeCandlesticksParams, WaitMarketOutcomeCandlesticksResult,
};
use fedimint_prediction_markets_common::{
    api::{
        GetMarketOutcomeCandlesticksParams, GetMarketOutcomeCandlesticksResult,
        GetPayoutControlMarketsParams, GetPayoutControlMarketsResult,
    },
    Market, Order,
};
use secp256k1::PublicKey;

#[apply(async_trait_maybe_send!)]
pub trait PredictionMarketsFederationApi {
    async fn get_market(&self, market: OutPoint) -> FederationResult<Option<Market>>;
    async fn get_order(&self, order: PublicKey) -> FederationResult<Option<Order>>;
    async fn get_payout_control_markets(
        &self,
        params: GetPayoutControlMarketsParams,
    ) -> FederationResult<GetPayoutControlMarketsResult>;
    async fn get_market_payout_control_proposals(
        &self,
        market: OutPoint,
    ) -> FederationResult<BTreeMap<PublicKey, Vec<Amount>>>;
    async fn get_market_outcome_candlesticks(
        &self,
        params: GetMarketOutcomeCandlesticksParams,
    ) -> FederationResult<GetMarketOutcomeCandlesticksResult>;
    async fn wait_market_outcome_candlesticks(
        &self,
        params: WaitMarketOutcomeCandlesticksParams,
    ) -> FederationResult<WaitMarketOutcomeCandlesticksResult>;
    async fn get_payout_control_balance(
        &self,
        payout_control: PublicKey,
    ) -> FederationResult<Amount>;
}

#[apply(async_trait_maybe_send!)]
impl<T: ?Sized> PredictionMarketsFederationApi for T
where
    T: IModuleFederationApi + MaybeSend + MaybeSync + 'static,
{
    async fn get_market(&self, market: OutPoint) -> FederationResult<Option<Market>> {
        self.request_current_consensus("get_market".to_string(), ApiRequestErased::new(market))
            .await
    }

    async fn get_order(&self, order: PublicKey) -> FederationResult<Option<Order>> {
        self.request_current_consensus("get_order".to_string(), ApiRequestErased::new(order))
            .await
    }

    async fn get_payout_control_markets(
        &self,
        params: GetPayoutControlMarketsParams,
    ) -> FederationResult<GetPayoutControlMarketsResult> {
        self.request_current_consensus(
            "get_payout_control_markets".to_string(),
            ApiRequestErased::new(params),
        )
        .await
    }

    async fn get_market_payout_control_proposals(
        &self,
        market: OutPoint,
    ) -> FederationResult<BTreeMap<PublicKey, Vec<Amount>>> {
        self.request_current_consensus(
            "get_market_payout_control_proposals".to_string(),
            ApiRequestErased::new(market),
        )
        .await
    }

    async fn get_market_outcome_candlesticks(
        &self,
        params: GetMarketOutcomeCandlesticksParams,
    ) -> FederationResult<GetMarketOutcomeCandlesticksResult> {
        self.request_current_consensus(
            "get_market_outcome_candlesticks".to_string(),
            ApiRequestErased::new(params),
        )
        .await
    }

    async fn wait_market_outcome_candlesticks(
        &self,
        params: WaitMarketOutcomeCandlesticksParams,
    ) -> FederationResult<WaitMarketOutcomeCandlesticksResult> {
        self.request_current_consensus(
            "wait_market_outcome_candlesticks".to_string(),
            ApiRequestErased::new(params),
        )
        .await
    }

    async fn get_payout_control_balance(
        &self,
        payout_control: PublicKey,
    ) -> FederationResult<Amount> {
        self.request_current_consensus(
            "get_payout_control_balance".to_string(),
            ApiRequestErased::new(payout_control),
        )
        .await
    }
}

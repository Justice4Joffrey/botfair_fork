// This file has been automatically generated by botfair
// from the Betfair APING documentation at
// https://docs.developer.betfair.com

#![allow(non_snake_case)]
use crate::generated_requests::*;
use crate::generated_types::*;
use crate::json_rpc::RpcRequest;
use crate::result::Result;
use chrono::{DateTime, Utc};
impl crate::BFClient {
    #[allow(dead_code)]
    pub fn listEventTypes(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<EventTypeResult>> {
        let req: listEventTypesRequest =
            listEventTypesRequest { filter, locale };
        let rpc_request: RpcRequest<listEventTypesRequest> =
            RpcRequest::new("SportsAPING/v1.0/listEventTypes".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listCompetitions(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<CompetitionResult>> {
        let req: listCompetitionsRequest =
            listCompetitionsRequest { filter, locale };
        let rpc_request: RpcRequest<listCompetitionsRequest> = RpcRequest::new(
            "SportsAPING/v1.0/listCompetitions".to_owned(),
            req,
        );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listTimeRanges(
        &self,
        filter: MarketFilter,
        granularity: TimeGranularity,
    ) -> Result<Vec<TimeRangeResult>> {
        let req: listTimeRangesRequest = listTimeRangesRequest {
            filter,
            granularity,
        };
        let rpc_request: RpcRequest<listTimeRangesRequest> =
            RpcRequest::new("SportsAPING/v1.0/listTimeRanges".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listEvents(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<EventResult>> {
        let req: listEventsRequest = listEventsRequest { filter, locale };
        let rpc_request: RpcRequest<listEventsRequest> =
            RpcRequest::new("SportsAPING/v1.0/listEvents".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listMarketTypes(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<MarketTypeResult>> {
        let req: listMarketTypesRequest =
            listMarketTypesRequest { filter, locale };
        let rpc_request: RpcRequest<listMarketTypesRequest> = RpcRequest::new(
            "SportsAPING/v1.0/listMarketTypes".to_owned(),
            req,
        );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listCountries(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<CountryCodeResult>> {
        let req: listCountriesRequest =
            listCountriesRequest { filter, locale };
        let rpc_request: RpcRequest<listCountriesRequest> =
            RpcRequest::new("SportsAPING/v1.0/listCountries".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listVenues(
        &self,
        filter: MarketFilter,
        locale: Option<String>,
    ) -> Result<Vec<VenueResult>> {
        let req: listVenuesRequest = listVenuesRequest { filter, locale };
        let rpc_request: RpcRequest<listVenuesRequest> =
            RpcRequest::new("SportsAPING/v1.0/listVenues".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listMarketCatalogue(
        &self,
        filter: MarketFilter,
        marketProjection: Option<Vec<MarketProjection>>,
        sort: Option<MarketSort>,
        maxResults: i32,
        locale: Option<String>,
    ) -> Result<Vec<MarketCatalogue>> {
        let req: listMarketCatalogueRequest = listMarketCatalogueRequest {
            filter,
            marketProjection,
            sort,
            maxResults,
            locale,
        };
        let rpc_request: RpcRequest<listMarketCatalogueRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/listMarketCatalogue".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listMarketBook(
        &self,
        marketIds: Vec<MarketId>,
        priceProjection: Option<PriceProjection>,
        orderProjection: Option<OrderProjection>,
        matchProjection: Option<MatchProjection>,
        includeOverallPosition: Option<bool>,
        partitionMatchedByStrategyRef: Option<bool>,
        customerStrategyRefs: Option<Vec<String>>,
        currencyCode: Option<String>,
        locale: Option<String>,
        matchedSince: Option<DateTime<Utc>>,
        betIds: Option<Vec<BetId>>,
    ) -> Result<Vec<MarketBook>> {
        let req: listMarketBookRequest = listMarketBookRequest {
            marketIds,
            priceProjection,
            orderProjection,
            matchProjection,
            includeOverallPosition,
            partitionMatchedByStrategyRef,
            customerStrategyRefs,
            currencyCode,
            locale,
            matchedSince,
            betIds,
        };
        let rpc_request: RpcRequest<listMarketBookRequest> =
            RpcRequest::new("SportsAPING/v1.0/listMarketBook".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listRunnerBook(
        &self,
        marketId: MarketId,
        selectionId: SelectionId,
        handicap: Option<f64>,
        priceProjection: Option<PriceProjection>,
        orderProjection: Option<OrderProjection>,
        matchProjection: Option<MatchProjection>,
        includeOverallPosition: Option<bool>,
        partitionMatchedByStrategyRef: Option<bool>,
        customerStrategyRefs: Option<Vec<String>>,
        currencyCode: Option<String>,
        locale: Option<String>,
        matchedSince: Option<DateTime<Utc>>,
        betIds: Option<Vec<BetId>>,
    ) -> Result<Vec<MarketBook>> {
        let req: listRunnerBookRequest = listRunnerBookRequest {
            marketId,
            selectionId,
            handicap,
            priceProjection,
            orderProjection,
            matchProjection,
            includeOverallPosition,
            partitionMatchedByStrategyRef,
            customerStrategyRefs,
            currencyCode,
            locale,
            matchedSince,
            betIds,
        };
        let rpc_request: RpcRequest<listRunnerBookRequest> =
            RpcRequest::new("SportsAPING/v1.0/listRunnerBook".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listCurrentOrders(
        &self,
        betIds: Option<Vec<BetId>>,
        marketIds: Option<Vec<MarketId>>,
        orderProjection: Option<OrderProjection>,
        customerOrderRefs: Option<Vec<CustomerOrderRef>>,
        customerStrategyRefs: Option<Vec<CustomerStrategyRef>>,
        placedDateRange: Option<TimeRange>,
        dateRange: Option<TimeRange>,
        orderBy: Option<OrderBy>,
        sortDir: Option<SortDir>,
        fromRecord: Option<i32>,
        recordCount: Option<i32>,
    ) -> Result<CurrentOrderSummaryReport> {
        let req: listCurrentOrdersRequest = listCurrentOrdersRequest {
            betIds,
            marketIds,
            orderProjection,
            customerOrderRefs,
            customerStrategyRefs,
            placedDateRange,
            dateRange,
            orderBy,
            sortDir,
            fromRecord,
            recordCount,
        };
        let rpc_request: RpcRequest<listCurrentOrdersRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/listCurrentOrders".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listClearedOrders(
        &self,
        betStatus: BetStatus,
        eventTypeIds: Option<Vec<EventTypeId>>,
        eventIds: Option<Vec<EventId>>,
        marketIds: Option<Vec<MarketId>>,
        runnerIds: Option<Vec<RunnerId>>,
        betIds: Option<Vec<BetId>>,
        customerOrderRefs: Option<Vec<CustomerOrderRef>>,
        customerStrategyRefs: Option<Vec<CustomerStrategyRef>>,
        side: Option<Side>,
        settledDateRange: Option<TimeRange>,
        groupBy: Option<GroupBy>,
        includeItemDescription: Option<bool>,
        locale: Option<String>,
        fromRecord: Option<i32>,
        recordCount: Option<i32>,
    ) -> Result<ClearedOrderSummaryReport> {
        let req: listClearedOrdersRequest = listClearedOrdersRequest {
            betStatus,
            eventTypeIds,
            eventIds,
            marketIds,
            runnerIds,
            betIds,
            customerOrderRefs,
            customerStrategyRefs,
            side,
            settledDateRange,
            groupBy,
            includeItemDescription,
            locale,
            fromRecord,
            recordCount,
        };
        let rpc_request: RpcRequest<listClearedOrdersRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/listClearedOrders".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn placeOrders(
        &self,
        marketId: MarketId,
        instructions: Vec<PlaceInstruction>,
        customerRef: Option<String>,
        marketVersion: Option<MarketVersion>,
        customerStrategyRef: Option<String>,
        r#async: Option<bool>,
    ) -> Result<PlaceExecutionReport> {
        let req: placeOrdersRequest = placeOrdersRequest {
            marketId,
            instructions,
            customerRef,
            marketVersion,
            customerStrategyRef,
            r#async,
        };
        let rpc_request: RpcRequest<placeOrdersRequest> =
            RpcRequest::new("SportsAPING/v1.0/placeOrders".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn cancelOrders(
        &self,
        marketId: Option<MarketId>,
        instructions: Option<Vec<CancelInstruction>>,
        customerRef: Option<String>,
    ) -> Result<CancelExecutionReport> {
        let req: cancelOrdersRequest = cancelOrdersRequest {
            marketId,
            instructions,
            customerRef,
        };
        let rpc_request: RpcRequest<cancelOrdersRequest> =
            RpcRequest::new("SportsAPING/v1.0/cancelOrders".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn replaceOrders(
        &self,
        marketId: MarketId,
        instructions: Vec<ReplaceInstruction>,
        customerRef: Option<String>,
        marketVersion: Option<MarketVersion>,
        r#async: Option<bool>,
    ) -> Result<ReplaceExecutionReport> {
        let req: replaceOrdersRequest = replaceOrdersRequest {
            marketId,
            instructions,
            customerRef,
            marketVersion,
            r#async,
        };
        let rpc_request: RpcRequest<replaceOrdersRequest> =
            RpcRequest::new("SportsAPING/v1.0/replaceOrders".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn updateOrders(
        &self,
        marketId: MarketId,
        instructions: Vec<UpdateInstruction>,
        customerRef: Option<String>,
    ) -> Result<UpdateExecutionReport> {
        let req: updateOrdersRequest = updateOrdersRequest {
            marketId,
            instructions,
            customerRef,
        };
        let rpc_request: RpcRequest<updateOrdersRequest> =
            RpcRequest::new("SportsAPING/v1.0/updateOrders".to_owned(), req);
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listMarketProfitAndLoss(
        &self,
        marketIds: Vec<MarketId>,
        includeSettledBets: Option<bool>,
        includeBspBets: Option<bool>,
        netOfCommission: Option<bool>,
    ) -> Result<Vec<MarketProfitAndLoss>> {
        let req: listMarketProfitAndLossRequest =
            listMarketProfitAndLossRequest {
                marketIds,
                includeSettledBets,
                includeBspBets,
                netOfCommission,
            };
        let rpc_request: RpcRequest<listMarketProfitAndLossRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/listMarketProfitAndLoss".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn setDefaultExposureLimitForMarketGroups(
        &self,
        marketGroupType: MarketGroupType,
        limit: ExposureLimit,
    ) -> Result<String> {
        let req: setDefaultExposureLimitForMarketGroupsRequest =
            setDefaultExposureLimitForMarketGroupsRequest {
                marketGroupType,
                limit,
            };
        let rpc_request: RpcRequest<
            setDefaultExposureLimitForMarketGroupsRequest,
        > = RpcRequest::new(
            "SportsAPING/v1.0/setDefaultExposureLimitForMarketGroups"
                .to_owned(),
            req,
        );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn setExposureLimitForMarketGroup(
        &self,
        marketGroup: MarketGroup,
        limit: ExposureLimit,
    ) -> Result<String> {
        let req: setExposureLimitForMarketGroupRequest =
            setExposureLimitForMarketGroupRequest { marketGroup, limit };
        let rpc_request: RpcRequest<setExposureLimitForMarketGroupRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/setExposureLimitForMarketGroup".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn removeDefaultExposureLimitForMarketGroups(
        &self,
        marketGroupType: MarketGroupType,
    ) -> Result<String> {
        let req: removeDefaultExposureLimitForMarketGroupsRequest =
            removeDefaultExposureLimitForMarketGroupsRequest {
                marketGroupType,
            };
        let rpc_request: RpcRequest<
            removeDefaultExposureLimitForMarketGroupsRequest,
        > = RpcRequest::new(
            "SportsAPING/v1.0/removeDefaultExposureLimitForMarketGroups"
                .to_owned(),
            req,
        );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn removeExposureLimitForMarketGroup(
        &self,
        marketGroup: MarketGroup,
    ) -> Result<String> {
        let req: removeExposureLimitForMarketGroupRequest =
            removeExposureLimitForMarketGroupRequest { marketGroup };
        let rpc_request: RpcRequest<removeExposureLimitForMarketGroupRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/removeExposureLimitForMarketGroup"
                    .to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn listExposureLimitsForMarketGroups(
        &self,
        marketGroupTypeFilter: Option<MarketGroupType>,
        marketGroupFilter: Option<Vec<MarketGroup>>,
    ) -> Result<Vec<ExposureLimitsForMarketGroups>> {
        let req: listExposureLimitsForMarketGroupsRequest =
            listExposureLimitsForMarketGroupsRequest {
                marketGroupTypeFilter,
                marketGroupFilter,
            };
        let rpc_request: RpcRequest<listExposureLimitsForMarketGroupsRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/listExposureLimitsForMarketGroups"
                    .to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn unblockMarketGroup(
        &self,
        marketGroup: MarketGroup,
    ) -> Result<String> {
        let req: unblockMarketGroupRequest =
            unblockMarketGroupRequest { marketGroup };
        let rpc_request: RpcRequest<unblockMarketGroupRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/unblockMarketGroup".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn getExposureReuseEnabledEvents(&self) -> Result<Vec<i64>> {
        let rpc_request: RpcRequest<()> = RpcRequest::new(
            "SportsAPING/v1.0/getExposureReuseEnabledEvents".to_owned(),
            (),
        );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn addExposureReuseEnabledEvents(
        &self,
        eventIds: Vec<i64>,
    ) -> Result<String> {
        let req: addExposureReuseEnabledEventsRequest =
            addExposureReuseEnabledEventsRequest { eventIds };
        let rpc_request: RpcRequest<addExposureReuseEnabledEventsRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/addExposureReuseEnabledEvents".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
    #[allow(dead_code)]
    pub fn removeExposureReuseEnabledEvents(
        &self,
        eventIds: Vec<i64>,
    ) -> Result<String> {
        let req: removeExposureReuseEnabledEventsRequest =
            removeExposureReuseEnabledEventsRequest { eventIds };
        let rpc_request: RpcRequest<removeExposureReuseEnabledEventsRequest> =
            RpcRequest::new(
                "SportsAPING/v1.0/removeExposureReuseEnabledEvents".to_owned(),
                req,
            );
        self.req(rpc_request).map(|x| x.into_inner())
    }
}

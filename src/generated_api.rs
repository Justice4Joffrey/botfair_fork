// This file is generated.
// Any manual edits will be overwritten.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
use crate::json_rpc::{RpcRequest, RpcResponse};
use crate::AnyError;
use chrono::{DateTime, Utc};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize)]
pub struct listEventTypesRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listEventTypes(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<EventTypeResult>, AnyError> {
    let req: listEventTypesRequest = listEventTypesRequest { filter, locale };
    let rpc_request: RpcRequest<listEventTypesRequest> =
        RpcRequest::new("SportsAPING/v1.0/listEventTypes".to_owned(), req);
    let resp: RpcResponse<Vec<EventTypeResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listCompetitionsRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listCompetitions(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<CompetitionResult>, AnyError> {
    let req: listCompetitionsRequest =
        listCompetitionsRequest { filter, locale };
    let rpc_request: RpcRequest<listCompetitionsRequest> =
        RpcRequest::new("SportsAPING/v1.0/listCompetitions".to_owned(), req);
    let resp: RpcResponse<Vec<CompetitionResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listTimeRangesRequest {
    pub filter: MarketFilter,
    pub granularity: TimeGranularity,
}

pub fn listTimeRanges(
    rb: RequestBuilder,
    filter: MarketFilter,
    granularity: TimeGranularity,
) -> Result<Vec<TimeRangeResult>, AnyError> {
    let req: listTimeRangesRequest = listTimeRangesRequest {
        filter,
        granularity,
    };
    let rpc_request: RpcRequest<listTimeRangesRequest> =
        RpcRequest::new("SportsAPING/v1.0/listTimeRanges".to_owned(), req);
    let resp: RpcResponse<Vec<TimeRangeResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listEventsRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listEvents(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<EventResult>, AnyError> {
    let req: listEventsRequest = listEventsRequest { filter, locale };
    let rpc_request: RpcRequest<listEventsRequest> =
        RpcRequest::new("SportsAPING/v1.0/listEvents".to_owned(), req);
    let resp: RpcResponse<Vec<EventResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listMarketTypesRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listMarketTypes(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<MarketTypeResult>, AnyError> {
    let req: listMarketTypesRequest =
        listMarketTypesRequest { filter, locale };
    let rpc_request: RpcRequest<listMarketTypesRequest> =
        RpcRequest::new("SportsAPING/v1.0/listMarketTypes".to_owned(), req);
    let resp: RpcResponse<Vec<MarketTypeResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listCountriesRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listCountries(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<CountryCodeResult>, AnyError> {
    let req: listCountriesRequest = listCountriesRequest { filter, locale };
    let rpc_request: RpcRequest<listCountriesRequest> =
        RpcRequest::new("SportsAPING/v1.0/listCountries".to_owned(), req);
    let resp: RpcResponse<Vec<CountryCodeResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listVenuesRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listVenues(
    rb: RequestBuilder,
    filter: MarketFilter,
    locale: Option<String>,
) -> Result<Vec<VenueResult>, AnyError> {
    let req: listVenuesRequest = listVenuesRequest { filter, locale };
    let rpc_request: RpcRequest<listVenuesRequest> =
        RpcRequest::new("SportsAPING/v1.0/listVenues".to_owned(), req);
    let resp: RpcResponse<Vec<VenueResult>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listMarketCatalogueRequest {
    pub filter: MarketFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketProjection: Option<Vec<MarketProjection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MarketSort>,
    pub maxResults: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

pub fn listMarketCatalogue(
    rb: RequestBuilder,
    filter: MarketFilter,
    marketProjection: Option<Vec<MarketProjection>>,
    sort: Option<MarketSort>,
    maxResults: i32,
    locale: Option<String>,
) -> Result<Vec<MarketCatalogue>, AnyError> {
    let req: listMarketCatalogueRequest = listMarketCatalogueRequest {
        filter,
        marketProjection,
        sort,
        maxResults,
        locale,
    };
    let rpc_request: RpcRequest<listMarketCatalogueRequest> = RpcRequest::new(
        "SportsAPING/v1.0/listMarketCatalogue".to_owned(),
        req,
    );
    let resp: RpcResponse<Vec<MarketCatalogue>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listMarketBookRequest {
    pub marketIds: Vec<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceProjection: Option<PriceProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderProjection: Option<OrderProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchProjection: Option<MatchProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeOverallPosition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionMatchedByStrategyRef: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRefs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchedSince: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betIds: Option<Vec<BetId>>,
}

pub fn listMarketBook(
    rb: RequestBuilder,
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
) -> Result<Vec<MarketBook>, AnyError> {
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
    let resp: RpcResponse<Vec<MarketBook>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listRunnerBookRequest {
    pub marketId: MarketId,
    pub selectionId: SelectionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handicap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceProjection: Option<PriceProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderProjection: Option<OrderProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchProjection: Option<MatchProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeOverallPosition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionMatchedByStrategyRef: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRefs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchedSince: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betIds: Option<Vec<BetId>>,
}

pub fn listRunnerBook(
    rb: RequestBuilder,
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
) -> Result<Vec<MarketBook>, AnyError> {
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
    let resp: RpcResponse<Vec<MarketBook>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listCurrentOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betIds: Option<Vec<BetId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketIds: Option<Vec<MarketId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderProjection: Option<OrderProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRefs: Option<Vec<CustomerOrderRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRefs: Option<Vec<CustomerStrategyRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placedDateRange: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateRange: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderBy: Option<OrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortDir: Option<SortDir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromRecord: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recordCount: Option<i32>,
}

pub fn listCurrentOrders(
    rb: RequestBuilder,
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
) -> Result<CurrentOrderSummaryReport, AnyError> {
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
        RpcRequest::new("SportsAPING/v1.0/listCurrentOrders".to_owned(), req);
    let resp: RpcResponse<CurrentOrderSummaryReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listClearedOrdersRequest {
    pub betStatus: BetStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventTypeIds: Option<Vec<EventTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventIds: Option<Vec<EventId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketIds: Option<Vec<MarketId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnerIds: Option<Vec<RunnerId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betIds: Option<Vec<BetId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRefs: Option<Vec<CustomerOrderRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRefs: Option<Vec<CustomerStrategyRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settledDateRange: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupBy: Option<GroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeItemDescription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromRecord: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recordCount: Option<i32>,
}

pub fn listClearedOrders(
    rb: RequestBuilder,
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
) -> Result<ClearedOrderSummaryReport, AnyError> {
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
        RpcRequest::new("SportsAPING/v1.0/listClearedOrders".to_owned(), req);
    let resp: RpcResponse<ClearedOrderSummaryReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct placeOrdersRequest {
    pub marketId: MarketId,
    pub instructions: Vec<PlaceInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketVersion: Option<MarketVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRef: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
}

pub fn placeOrders(
    rb: RequestBuilder,
    marketId: MarketId,
    instructions: Vec<PlaceInstruction>,
    customerRef: Option<String>,
    marketVersion: Option<MarketVersion>,
    customerStrategyRef: Option<String>,
    r#async: Option<bool>,
) -> Result<PlaceExecutionReport, AnyError> {
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
    let resp: RpcResponse<PlaceExecutionReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct cancelOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Vec<CancelInstruction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
}

pub fn cancelOrders(
    rb: RequestBuilder,
    marketId: Option<MarketId>,
    instructions: Option<Vec<CancelInstruction>>,
    customerRef: Option<String>,
) -> Result<CancelExecutionReport, AnyError> {
    let req: cancelOrdersRequest = cancelOrdersRequest {
        marketId,
        instructions,
        customerRef,
    };
    let rpc_request: RpcRequest<cancelOrdersRequest> =
        RpcRequest::new("SportsAPING/v1.0/cancelOrders".to_owned(), req);
    let resp: RpcResponse<CancelExecutionReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct replaceOrdersRequest {
    pub marketId: MarketId,
    pub instructions: Vec<ReplaceInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketVersion: Option<MarketVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
}

pub fn replaceOrders(
    rb: RequestBuilder,
    marketId: MarketId,
    instructions: Vec<ReplaceInstruction>,
    customerRef: Option<String>,
    marketVersion: Option<MarketVersion>,
    r#async: Option<bool>,
) -> Result<ReplaceExecutionReport, AnyError> {
    let req: replaceOrdersRequest = replaceOrdersRequest {
        marketId,
        instructions,
        customerRef,
        marketVersion,
        r#async,
    };
    let rpc_request: RpcRequest<replaceOrdersRequest> =
        RpcRequest::new("SportsAPING/v1.0/replaceOrders".to_owned(), req);
    let resp: RpcResponse<ReplaceExecutionReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct updateOrdersRequest {
    pub marketId: MarketId,
    pub instructions: Vec<UpdateInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
}

pub fn updateOrders(
    rb: RequestBuilder,
    marketId: MarketId,
    instructions: Vec<UpdateInstruction>,
    customerRef: Option<String>,
) -> Result<UpdateExecutionReport, AnyError> {
    let req: updateOrdersRequest = updateOrdersRequest {
        marketId,
        instructions,
        customerRef,
    };
    let rpc_request: RpcRequest<updateOrdersRequest> =
        RpcRequest::new("SportsAPING/v1.0/updateOrders".to_owned(), req);
    let resp: RpcResponse<UpdateExecutionReport> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listMarketProfitAndLossRequest {
    pub marketIds: Vec<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeSettledBets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeBspBets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netOfCommission: Option<bool>,
}

pub fn listMarketProfitAndLoss(
    rb: RequestBuilder,
    marketIds: Vec<MarketId>,
    includeSettledBets: Option<bool>,
    includeBspBets: Option<bool>,
    netOfCommission: Option<bool>,
) -> Result<Vec<MarketProfitAndLoss>, AnyError> {
    let req: listMarketProfitAndLossRequest = listMarketProfitAndLossRequest {
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
    let resp: RpcResponse<Vec<MarketProfitAndLoss>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct setDefaultExposureLimitForMarketGroupsRequest {
    pub marketGroupType: MarketGroupType,
    pub limit: ExposureLimit,
}

pub fn setDefaultExposureLimitForMarketGroups(
    rb: RequestBuilder,
    marketGroupType: MarketGroupType,
    limit: ExposureLimit,
) -> Result<String, AnyError> {
    let req: setDefaultExposureLimitForMarketGroupsRequest =
        setDefaultExposureLimitForMarketGroupsRequest {
            marketGroupType,
            limit,
        };
    let rpc_request: RpcRequest<
        setDefaultExposureLimitForMarketGroupsRequest,
    > = RpcRequest::new(
        "SportsAPING/v1.0/setDefaultExposureLimitForMarketGroups".to_owned(),
        req,
    );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct setExposureLimitForMarketGroupRequest {
    pub marketGroup: MarketGroup,
    pub limit: ExposureLimit,
}

pub fn setExposureLimitForMarketGroup(
    rb: RequestBuilder,
    marketGroup: MarketGroup,
    limit: ExposureLimit,
) -> Result<String, AnyError> {
    let req: setExposureLimitForMarketGroupRequest =
        setExposureLimitForMarketGroupRequest { marketGroup, limit };
    let rpc_request: RpcRequest<setExposureLimitForMarketGroupRequest> =
        RpcRequest::new(
            "SportsAPING/v1.0/setExposureLimitForMarketGroup".to_owned(),
            req,
        );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct removeDefaultExposureLimitForMarketGroupsRequest {
    pub marketGroupType: MarketGroupType,
}

pub fn removeDefaultExposureLimitForMarketGroups(
    rb: RequestBuilder,
    marketGroupType: MarketGroupType,
) -> Result<String, AnyError> {
    let req: removeDefaultExposureLimitForMarketGroupsRequest =
        removeDefaultExposureLimitForMarketGroupsRequest { marketGroupType };
    let rpc_request: RpcRequest<
        removeDefaultExposureLimitForMarketGroupsRequest,
    > = RpcRequest::new(
        "SportsAPING/v1.0/removeDefaultExposureLimitForMarketGroups"
            .to_owned(),
        req,
    );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct removeExposureLimitForMarketGroupRequest {
    pub marketGroup: MarketGroup,
}

pub fn removeExposureLimitForMarketGroup(
    rb: RequestBuilder,
    marketGroup: MarketGroup,
) -> Result<String, AnyError> {
    let req: removeExposureLimitForMarketGroupRequest =
        removeExposureLimitForMarketGroupRequest { marketGroup };
    let rpc_request: RpcRequest<removeExposureLimitForMarketGroupRequest> =
        RpcRequest::new(
            "SportsAPING/v1.0/removeExposureLimitForMarketGroup".to_owned(),
            req,
        );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct listExposureLimitsForMarketGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketGroupTypeFilter: Option<MarketGroupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketGroupFilter: Option<Vec<MarketGroup>>,
}

pub fn listExposureLimitsForMarketGroups(
    rb: RequestBuilder,
    marketGroupTypeFilter: Option<MarketGroupType>,
    marketGroupFilter: Option<Vec<MarketGroup>>,
) -> Result<Vec<ExposureLimitsForMarketGroups>, AnyError> {
    let req: listExposureLimitsForMarketGroupsRequest =
        listExposureLimitsForMarketGroupsRequest {
            marketGroupTypeFilter,
            marketGroupFilter,
        };
    let rpc_request: RpcRequest<listExposureLimitsForMarketGroupsRequest> =
        RpcRequest::new(
            "SportsAPING/v1.0/listExposureLimitsForMarketGroups".to_owned(),
            req,
        );
    let resp: RpcResponse<Vec<ExposureLimitsForMarketGroups>> =
        rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct unblockMarketGroupRequest {
    pub marketGroup: MarketGroup,
}

pub fn unblockMarketGroup(
    rb: RequestBuilder,
    marketGroup: MarketGroup,
) -> Result<String, AnyError> {
    let req: unblockMarketGroupRequest =
        unblockMarketGroupRequest { marketGroup };
    let rpc_request: RpcRequest<unblockMarketGroupRequest> =
        RpcRequest::new("SportsAPING/v1.0/unblockMarketGroup".to_owned(), req);
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

pub fn getExposureReuseEnabledEvents(
    rb: RequestBuilder,
) -> Result<Vec<i64>, AnyError> {
    let rpc_request: RpcRequest<()> = RpcRequest::new(
        "SportsAPING/v1.0/getExposureReuseEnabledEvents".to_owned(),
        (),
    );
    let resp: RpcResponse<Vec<i64>> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct addExposureReuseEnabledEventsRequest {
    pub eventIds: Vec<i64>,
}

pub fn addExposureReuseEnabledEvents(
    rb: RequestBuilder,
    eventIds: Vec<i64>,
) -> Result<String, AnyError> {
    let req: addExposureReuseEnabledEventsRequest =
        addExposureReuseEnabledEventsRequest { eventIds };
    let rpc_request: RpcRequest<addExposureReuseEnabledEventsRequest> =
        RpcRequest::new(
            "SportsAPING/v1.0/addExposureReuseEnabledEvents".to_owned(),
            req,
        );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}

#[derive(Serialize)]
pub struct removeExposureReuseEnabledEventsRequest {
    pub eventIds: Vec<i64>,
}

pub fn removeExposureReuseEnabledEvents(
    rb: RequestBuilder,
    eventIds: Vec<i64>,
) -> Result<String, AnyError> {
    let req: removeExposureReuseEnabledEventsRequest =
        removeExposureReuseEnabledEventsRequest { eventIds };
    let rpc_request: RpcRequest<removeExposureReuseEnabledEventsRequest> =
        RpcRequest::new(
            "SportsAPING/v1.0/removeExposureReuseEnabledEvents".to_owned(),
            req,
        );
    let resp: RpcResponse<String> = rb.json(&rpc_request).send()?.json()?;
    Ok(resp.into_inner())
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MarketProjection {
    COMPETITION,
    EVENT,
    EVENT_TYPE,
    MARKET_START_TIME,
    MARKET_DESCRIPTION,
    RUNNER_DESCRIPTION,
    RUNNER_METADATA,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PriceData {
    SP_AVAILABLE,
    SP_TRADED,
    EX_BEST_OFFERS,
    EX_ALL_OFFERS,
    EX_TRADED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MatchProjection {
    NO_ROLLUP,
    ROLLED_UP_BY_PRICE,
    ROLLED_UP_BY_AVG_PRICE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum OrderProjection {
    ALL,
    EXECUTABLE,
    EXECUTION_COMPLETE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MarketStatus {
    INACTIVE,
    OPEN,
    SUSPENDED,
    CLOSED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RunnerStatus {
    ACTIVE,
    WINNER,
    LOSER,
    REMOVED_VACANT,
    REMOVED,
    PLACED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum TimeGranularity {
    DAYS,
    HOURS,
    MINUTES,
}
pub type MarketType = String;
pub type Venue = String;
pub type MarketId = String;
pub type SelectionId = i64;
pub type Handicap = f64;
pub type EventId = String;
pub type EventTypeId = String;
pub type CountryCode = String;
pub type ExchangeId = String;
pub type CompetitionId = String;
pub type Price = f64;
pub type Size = f64;
pub type BetId = String;
pub type MatchId = String;
pub type CustomerOrderRef = String;
pub type CustomerStrategyRef = String;
#[derive(Debug, Deserialize, Serialize)]
pub enum Side {
    BACK,
    LAY,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum OrderStatus {
    PENDING,
    EXECUTION_COMPLETE,
    EXECUTABLE,
    EXPIRED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum OrderBy {
    BY_BET,
    BY_MARKET,
    BY_PLACE_TIME,
    BY_MATCH_TIME,
    BY_VOID_TIME,
    BY_SETTLED_TIME,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SortDir {
    EARLIEST_TO_LATEST,
    LATEST_TO_EARLIEST,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum OrderType {
    LIMIT,
    LIMIT_ON_CLOSE,
    MARKET_ON_CLOSE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MarketSort {
    MINIMUM_TRADED,
    MAXIMUM_TRADED,
    MINIMUM_AVAILABLE,
    MAXIMUM_AVAILABLE,
    FIRST_TO_START,
    LAST_TO_START,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MarketBettingType {
    ODDS,
    LINE,
    RANGE,
    ASIAN_HANDICAP_DOUBLE_LINE,
    ASIAN_HANDICAP_SINGLE_LINE,
    FIXED_ODDS,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ExecutionReportStatus {
    SUCCESS,
    FAILURE,
    PROCESSED_WITH_ERRORS,
    TIMEOUT,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ExecutionReportErrorCode {
    ERROR_IN_MATCHER,
    PROCESSED_WITH_ERRORS,
    BET_ACTION_ERROR,
    INVALID_ACCOUNT_STATE,
    INVALID_WALLET_STATUS,
    INSUFFICIENT_FUNDS,
    LOSS_LIMIT_EXCEEDED,
    MARKET_SUSPENDED,
    MARKET_NOT_OPEN_FOR_BETTING,
    DUPLICATE_TRANSACTION,
    INVALID_ORDER,
    INVALID_MARKET_ID,
    PERMISSION_DENIED,
    DUPLICATE_BETIDS,
    NO_ACTION_REQUIRED,
    SERVICE_UNAVAILABLE,
    REJECTED_BY_REGULATOR,
    NO_CHASING,
    REGULATOR_IS_NOT_AVAILABLE,
    TOO_MANY_INSTRUCTIONS,
    INVALID_MARKET_VERSION,
    EVENT_EXPOSURE_LIMIT_EXCEEDED,
    EVENT_MATCHED_EXPOSURE_LIMIT_EXCEEDED,
    EVENT_BLOCKED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PersistenceType {
    LAPSE,
    PERSIST,
    MARKET_ON_CLOSE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum InstructionReportStatus {
    SUCCESS,
    FAILURE,
    TIMEOUT,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum InstructionReportErrorCode {
    INVALID_BET_SIZE,
    INVALID_RUNNER,
    BET_TAKEN_OR_LAPSED,
    BET_IN_PROGRESS,
    RUNNER_REMOVED,
    MARKET_NOT_OPEN_FOR_BETTING,
    LOSS_LIMIT_EXCEEDED,
    MARKET_NOT_OPEN_FOR_BSP_BETTING,
    INVALID_PRICE_EDIT,
    INVALID_ODDS,
    INSUFFICIENT_FUNDS,
    INVALID_PERSISTENCE_TYPE,
    ERROR_IN_MATCHER,
    INVALID_BACK_LAY_COMBINATION,
    ERROR_IN_ORDER,
    INVALID_BID_TYPE,
    INVALID_BET_ID,
    CANCELLED_NOT_PLACED,
    RELATED_ACTION_FAILED,
    NO_ACTION_REQUIRED,
    INVALID_MIN_FILL_SIZE,
    INVALID_CUSTOMER_ORDER_REF,
    TIME_IN_FORCE_CONFLICT,
    UNEXPECTED_PERSISTENCE_TYPE,
    INVALID_ORDER_TYPE,
    UNEXPECTED_MIN_FILL_SIZE,
    INVALID_CUSTOMER_STRATEGY_REF,
    BET_LAPSED_PRICE_IMPROVEMENT_TOO_LARGE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RollupModel {
    STAKE,
    PAYOUT,
    MANAGED_LIABILITY,
    NONE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum GroupBy {
    EVENT_TYPE,
    EVENT,
    MARKET,
    RUNNER,
    SIDE,
    BET,
    STRATEGY,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum BetStatus {
    SETTLED,
    VOIDED,
    LAPSED,
    CANCELLED,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum TimeInForce {
    FILL_OR_KILL,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum BetTargetType {
    PAYOUT,
    BACKERS_PROFIT,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PriceLadderType {
    CLASSIC,
    FINEST,
    LINE_RANGE,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MarketGroupType {
    EVENT,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum LimitBreachActionType {
    REJECT_BETS,
    STOP_BETTING,
    TEAR_DOWN_MARKET_GROUP,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MarketFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textQuery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchangeIds: Option<Vec<ExchangeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventTypeIds: Option<Vec<EventTypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventIds: Option<Vec<EventId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitionIds: Option<Vec<CompetitionId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketIds: Option<Vec<MarketId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venues: Option<Vec<Venue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bspOnly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turnInPlayEnabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inPlayOnly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketBettingTypes: Option<Vec<MarketBettingType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCountries: Option<Vec<CountryCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketTypeCodes: Option<Vec<MarketType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketStartTime: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withOrders: Option<Vec<OrderStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raceTypes: Option<Vec<String>>,
}
/// Information about a market
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketCatalogue {
    pub marketId: String,
    pub marketName: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketStartTime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MarketDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalMatched: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runners: Option<Vec<RunnerCatalog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventType: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competition: Option<Competition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
}
/// The dynamic data in a market
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketBook {
    pub marketId: String,
    pub isMarketDataDelayed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betDelay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bspReconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inplay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numberOfWinners: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numberOfRunners: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numberOfActiveRunners: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastMatchTime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalMatched: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalAvailable: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossMatching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnersVoidable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runners: Option<Vec<Runner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyLineDescription: Option<KeyLineDescription>,
}
/// Information about the Runners (selections) in a market
#[derive(Debug, Deserialize, Serialize)]
pub struct RunnerCatalog {
    pub selectionId: SelectionId,
    pub runnerName: String,
    pub handicap: f64,
    pub sortPriority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
/// The dynamic data about runners in a market
#[derive(Debug, Deserialize, Serialize)]
pub struct Runner {
    pub selectionId: SelectionId,
    pub handicap: f64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustmentFactor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastPriceTraded: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalMatched: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removalDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp: Option<StartingPrices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex: Option<ExchangePrices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<Match>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchesByStrategy: Option<HashMap<String, Matches>>,
}
/// Information about the Betfair Starting Price. Only available in BSP markets
#[derive(Debug, Deserialize, Serialize)]
pub struct StartingPrices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearPrice: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farPrice: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backStakeTaken: Option<Vec<PriceSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layLiabilityTaken: Option<Vec<PriceSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualSP: Option<f64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangePrices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availableToBack: Option<Vec<PriceSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availableToLay: Option<Vec<PriceSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradedVolume: Option<Vec<PriceSize>>,
}
/// Event
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#id: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countryCode: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openDate: Option<DateTime<Utc>>,
}
/// Event Result
#[derive(Debug, Deserialize, Serialize)]
pub struct EventResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
/// Competition
#[derive(Debug, Deserialize, Serialize)]
pub struct Competition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#id: Option<CompetitionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// Competition Result
#[derive(Debug, Deserialize, Serialize)]
pub struct CompetitionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competition: Option<Competition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitionRegion: Option<String>,
}
/// EventType
#[derive(Debug, Deserialize, Serialize)]
pub struct EventType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#id: Option<EventTypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// EventType Result
#[derive(Debug, Deserialize, Serialize)]
pub struct EventTypeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventType: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
/// MarketType Result
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketTypeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketType: Option<MarketType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
/// CountryCode Result
#[derive(Debug, Deserialize, Serialize)]
pub struct CountryCodeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countryCode: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
/// Venue Result
#[derive(Debug, Deserialize, Serialize)]
pub struct VenueResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
/// TimeRange
#[derive(Debug, Deserialize, Serialize)]
pub struct TimeRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,
}
/// TimeRange Result
#[derive(Debug, Deserialize, Serialize)]
pub struct TimeRangeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeRange: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketCount: Option<i32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub betId: BetId,
    pub orderType: String,
    pub status: String,
    pub persistenceType: String,
    pub side: String,
    pub price: Price,
    pub size: Size,
    pub bspLiability: Size,
    pub placedDate: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgPriceMatched: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeMatched: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeRemaining: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeLapsed: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeCancelled: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeVoided: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRef: Option<CustomerOrderRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRef: Option<CustomerStrategyRef>,
}
/// Match list.
#[derive(Debug, Deserialize, Serialize)]
pub struct Matches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<Match>>,
}
/// An individual bet Match, or rollup by price or avg price. Rollup depends on the requested MatchProjection
#[derive(Debug, Deserialize, Serialize)]
pub struct Match {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betId: Option<BetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchId: Option<MatchId>,
    pub side: String,
    pub price: Price,
    pub Size: Size,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchDate: Option<DateTime<Utc>>,
}
/// Market definition
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketState {
    pub status: String,
    pub betDelay: i32,
    pub bspReconciled: bool,
    pub complete: bool,
    pub inplay: bool,
    pub numberOfActiveRunners: i32,
    pub lastMatchTime: DateTime<Utc>,
    pub totalMatched: Size,
    pub totalAvailable: Size,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyLineDescription: Option<KeyLineDescription>,
}
/// Market version
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
/// Market definition
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketDescription {
    pub persistenceEnabled: bool,
    pub bspMarket: bool,
    pub marketTime: DateTime<Utc>,
    pub suspendTime: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settleTime: Option<DateTime<Utc>>,
    pub bettingType: String,
    pub turnInPlayEnabled: bool,
    pub marketType: String,
    pub regulator: String,
    pub marketBaseRate: f64,
    pub discountAllowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesHasDate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarifications: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eachWayDivisor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineRangeInfo: Option<MarketLineRangeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raceType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceLadderDescription: Option<PriceLadderDescription>,
}
/// Market Rates
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketRates {
    pub marketBaseRate: f64,
    pub discountAllowed: bool,
}
/// Market Licence
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketLicence {
    pub wallet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesHasDate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarifications: Option<String>,
}
/// Market Line and Range Info
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketLineRangeInfo {
    pub maxUnitValue: f64,
    pub minUnitValue: f64,
    pub interval: f64,
    pub marketUnit: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceSize {
    pub price: Price,
    pub size: Size,
}
/// A container representing search results.
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentOrderSummaryReport {
    pub currentOrders: Vec<CurrentOrderSummary>,
    pub moreAvailable: bool,
}
/// Summary of a current order.
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentOrderSummary {
    pub betId: BetId,
    pub marketId: MarketId,
    pub selectionId: SelectionId,
    pub handicap: Handicap,
    pub priceSize: PriceSize,
    pub bspLiability: Size,
    pub side: String,
    pub status: String,
    pub persistenceType: String,
    pub orderType: String,
    pub placedDate: DateTime<Utc>,
    pub matchedDate: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub averagePriceMatched: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeMatched: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeRemaining: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeLapsed: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeCancelled: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeVoided: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatorAuthCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRef: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRef: Option<String>,
}
/// Summary of a cleared order.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClearedOrderSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventTypeId: Option<EventTypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventId: Option<EventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectionId: Option<SelectionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handicap: Option<Handicap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betId: Option<BetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placedDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistenceType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itemDescription: Option<ItemDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betOutcome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceRequested: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settledDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastMatchedDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betCount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceMatched: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceReduced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeSettled: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeCancelled: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRef: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerStrategyRef: Option<String>,
}
/// A container representing search results.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClearedOrderSummaryReport {
    pub clearedOrders: Vec<ClearedOrderSummary>,
    pub moreAvailable: bool,
}
/// This object contains some text which may be useful to render a betting history view. It offers no long-term warranty as to the correctness of the text.
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventTypeDesc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventDesc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketDesc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketStartTime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnerDesc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numberOfWinners: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eachWayDivisor: Option<f64>,
}
/// This object contains the unique identifier for a runner
#[derive(Debug, Deserialize, Serialize)]
pub struct RunnerId {
    pub marketId: MarketId,
    pub selectionId: SelectionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handicap: Option<Handicap>,
}
/// Instruction to place a new order
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaceInstruction {
    pub orderType: OrderType,
    pub selectionId: SelectionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handicap: Option<Handicap>,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitOrder: Option<LimitOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitOnCloseOrder: Option<LimitOnCloseOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketOnCloseOrder: Option<MarketOnCloseOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerOrderRef: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaceExecutionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionReports: Option<Vec<PlaceInstructionReport>>,
}
/// Place a new LIMIT order (simple exchange bet for immediate execution)
#[derive(Debug, Deserialize, Serialize)]
pub struct LimitOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    pub price: Price,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistenceType: Option<PersistenceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeInForce: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minFillSize: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betTargetType: Option<BetTargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betTargetSize: Option<Size>,
}
/// Place a new LIMIT_ON_CLOSE bet
#[derive(Debug, Deserialize, Serialize)]
pub struct LimitOnCloseOrder {
    pub liability: Size,
    pub price: Price,
}
/// Place a new MARKET_ON_CLOSE bet
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketOnCloseOrder {
    pub liability: Size,
}
/// Response to a PlaceInstruction
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaceInstructionReport {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderStatus: Option<OrderStatus>,
    pub instruction: PlaceInstruction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placedDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub averagePriceMatched: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeMatched: Option<Size>,
}
/// Instruction to fully or partially cancel an order (only applies to LIMIT orders)
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelInstruction {
    pub betId: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizeReduction: Option<Size>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelExecutionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionReports: Option<Vec<CancelInstructionReport>>,
}
/// Instruction to replace a LIMIT or LIMIT_ON_CLOSE order at a new price. Original order will be cancelled and a new order placed at the new price for the remaining stake.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceInstruction {
    pub betId: String,
    pub newPrice: Price,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceExecutionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionReports: Option<Vec<ReplaceInstructionReport>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceInstructionReport {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelInstructionReport: Option<CancelInstructionReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeInstructionReport: Option<PlaceInstructionReport>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelInstructionReport {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<CancelInstruction>,
    pub sizeCancelled: Size,
    pub cancelledDate: DateTime<Utc>,
}
/// Instruction to update LIMIT bet's persistence of an order that do not affect exposure
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateInstruction {
    pub betId: String,
    pub newPersistenceType: PersistenceType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateExecutionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customerRef: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<MarketId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionReports: Option<Vec<UpdateInstructionReport>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateInstructionReport {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorCode: Option<String>,
    pub instruction: UpdateInstruction,
}
/// Selection criteria of the returning price data
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceProjection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceData: Option<Vec<PriceData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exBestOffersOverrides: Option<ExBestOffersOverrides>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolloverStakes: Option<bool>,
}
/// Options to alter the default representation of best offer prices
#[derive(Debug, Deserialize, Serialize)]
pub struct ExBestOffersOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bestPricesDepth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollupModel: Option<RollupModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollupLimit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollupLiabilityThreshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollupLiabilityFactor: Option<i32>,
}
/// Profit and loss in a market
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketProfitAndLoss {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissionApplied: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profitAndLosses: Option<Vec<RunnerProfitAndLoss>>,
}
/// Profit and loss if selection is wins or loses
#[derive(Debug, Deserialize, Serialize)]
pub struct RunnerProfitAndLoss {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectionId: Option<SelectionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifWin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifLose: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifPlace: Option<f64>,
}
/// Description of the price ladder type and any related data.
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceLadderDescription {
    pub r#type: PriceLadderType,
}
/// Description of a markets key line selection, comprising the selectionId and handicap of the team it is applied to.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyLineSelection {
    pub selectionId: SelectionId,
    pub handicap: Handicap,
}
/// A list of KeyLineSelection objects describing the key line for the market
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyLineDescription {
    pub keyLine: Vec<KeyLineSelection>,
}
/// Wrapper type that contains accounts exposure limits for a market group type. If default limit exists for group type, defaultLimit value will be populated. Group limits to return can be controller by marketGroupFilter parameter (see listExposureLimitsForMarketGroups operation).
#[derive(Debug, Deserialize, Serialize)]
pub struct ExposureLimitsForMarketGroups {
    pub marketGroupType: MarketGroupType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultLimit: Option<ExposureLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupLimits: Option<Vec<MarketGroupExposureLimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockedMarketGroups: Option<Vec<MarketGroupId>>,
}
/// Represents a market group
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketGroup {
    pub r#type: MarketGroupType,
    pub r#id: MarketGroupId,
}
/// Container type for market group ID
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketGroupId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventId: Option<i64>,
}
/// Action that should be execute when limit is breached
#[derive(Debug, Deserialize, Serialize)]
pub struct LimitBreachAction {
    pub actionType: LimitBreachActionType,
}
/// Container type for a group exposure limit
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketGroupExposureLimit {
    pub groupId: MarketGroupId,
    pub limit: ExposureLimit,
}
/// Exposure limit and limit breach action. Not populating one of total or matched parameters indicates that no limit should be set for that exposure value.  A special use of this type is when none of its parameters are populated, this can be used to override default limit to "no limit" for a specific instance of market group (see setExposureLimitForMarketGroup operation)
#[derive(Debug, Deserialize, Serialize)]
pub struct ExposureLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitBreachAction: Option<LimitBreachAction>,
}

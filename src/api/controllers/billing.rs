use crate::api::controllers::macros::*;
use crate::api::types::billing::*;
use uuid::Uuid;

api_controller!(InfraBillingController);

api_get!(InfraBillingController, get_infra_providers, "/api/infra-billing/providers", GetInfraProvidersResponseDto);
api_post!(InfraBillingController, create_infra_provider, "/api/infra-billing/providers", CreateInfraProviderRequestDto, CreateInfraProviderResponseDto);
api_patch!(InfraBillingController, update_infra_provider, "/api/infra-billing/providers", UpdateInfraProviderRequestDto, UpdateInfraProviderResponseDto);
api_get_with_path!(InfraBillingController, get_infra_provider_by_uuid, "/api/infra-billing/providers/{}", GetInfraProviderByUuidResponseDto, uuid: Uuid);
api_delete!(InfraBillingController, delete_infra_provider_by_uuid, "/api/infra-billing/providers/{}", DeleteInfraProviderByUuidResponseDto, uuid: Uuid);

api_post!(
    InfraBillingController,
    create_infra_billing_history_record,
    "/api/infra-billing/history",
    CreateInfraBillingHistoryRecordRequestDto,
    CreateInfraBillingHistoryRecordResponseDto
);
api_get!(InfraBillingController, get_infra_billing_history_records, "/api/infra-billing/history", GetInfraBillingHistoryRecordsResponseDto);
api_delete!(InfraBillingController, delete_infra_billing_history_record_by_uuid, "/api/infra-billing/history/{}", DeleteInfraBillingHistoryRecordByUuidResponseDto, uuid: Uuid);

api_get!(InfraBillingController, get_billing_nodes, "/api/infra-billing/nodes", GetInfraBillingNodesResponseDto);
api_patch!(InfraBillingController, update_infra_billing_node, "/api/infra-billing/nodes", UpdateInfraBillingNodeRequestDto, UpdateInfraBillingNodeResponseDto);
api_post!(InfraBillingController, create_infra_billing_node, "/api/infra-billing/nodes", CreateInfraBillingNodeRequestDto, CreateInfraBillingNodeResponseDto);
api_delete!(InfraBillingController, delete_infra_billing_node_by_uuid, "/api/infra-billing/nodes/{}", DeleteInfraBillingNodeByUuidResponseDto, uuid: Uuid);

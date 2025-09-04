use crate::api::controllers::macros::*;
use crate::api::types::system::*;

api_controller!(SystemController);

api_get!(SystemController, get_stats, "/api/system/stats", GetStatsResponseDto);
api_get!(SystemController, get_bandwidth_stats, "/api/system/stats/bandwidth", GetBandwidthStatsResponseDto);
api_get!(SystemController, get_nodes_statistics, "/api/system/stats/nodes", GetNodesStatisticsResponseDto);
api_get!(SystemController, get_remnawave_health, "/api/system/health", GetRemnawaveHealthResponseDto);
api_get!(SystemController, get_nodes_metrics, "/api/system/nodes/metrics", GetNodesMetricsResponseDto);
api_get!(SystemController, get_x25519_keypairs, "/api/system/tools/x25519/generate", GenerateX25519ResponseDto);

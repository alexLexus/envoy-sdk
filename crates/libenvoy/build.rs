use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // proto/envoy/service/accesslog/v3/als.proto
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_well_known_types(true)
        .file_descriptor_set_path(out_dir.join("envoy.service.accesslog.v3.als.bin"))
        .compile_protos(&["proto/envoy/service/accesslog/v3/als.proto"], &["proto"])
        .unwrap();

    // proto/envoy/service/auth/v3/external_auth.proto
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_well_known_types(true)
        .file_descriptor_set_path(out_dir.join("envoy.service.auth.v3.external_auth.bin"))
        .compile_protos(
            &["proto/envoy/service/auth/v3/external_auth.proto"],
            &["proto"],
        )
        .unwrap();

    // proto/envoy/service/event_reporting/v3/event_reporting_service.proto
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_well_known_types(true)
        .file_descriptor_set_path(
            out_dir.join("proto.envoy.service.event_reporting.v3.event_reporting_service.bin"),
        )
        .compile_protos(
            &["proto/envoy/service/event_reporting/v3/event_reporting_service.proto"],
            &["proto"],
        )
        .unwrap();

    // proto/envoy/service/ext_proc/v3/external_processor.proto
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_well_known_types(true)
        .file_descriptor_set_path(
            out_dir.join("proto.envoy.service.ext_proc.v3.external_processor.bin"),
        )
        .compile_protos(
            &["proto/envoy/service/ext_proc/v3/external_processor.proto"],
            &["proto"],
        )
        .unwrap();

    // proto/envoy/service/network_ext_proc/v3/network_external_processor.proto
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_well_known_types(true)
        .file_descriptor_set_path(
            out_dir.join("proto.envoy.service.network_ext_proc.v3.network_external_processor.bin"),
        )
        .compile_protos(
            &["proto/envoy/service/network_ext_proc/v3/network_external_processor.proto"],
            &["proto"],
        )
        .unwrap();

    Ok(())
}

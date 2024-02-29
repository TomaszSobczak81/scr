fn main() {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/k8s")
        .compile(
            &["proto/k8s.io/cri-api/pkg/apis/runtime/v1/api.proto"],
            &["proto"],
        ).unwrap();
}

fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/k8s")
        .compile(
            &["proto/k8s.io/cri-api/pkg/apis/runtime/v1/api.proto"],
            &["proto"],
        ).unwrap();
}

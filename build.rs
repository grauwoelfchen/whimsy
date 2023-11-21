fn main() {
    #[rustfmt::skip]
    glib_build_tools::compile_resources(
        &["src/ui"],
        "src/ui/resource.xml",
        "whimsy.gresource",
    );
}

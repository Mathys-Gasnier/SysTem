use windres::Build;

fn main() {
    Build::new().compile("sys-tem.rc").unwrap();
}

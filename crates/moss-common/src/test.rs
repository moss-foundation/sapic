use ts_rs::TS;

struct Inner {}

#[derive(TS)]
#[ts(export, optional_fields)]
struct Test {
    #[ts(as = "Option<String>")]
    pub inner: Option<Inner>,
}

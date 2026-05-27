use std::any::TypeId;

use gds::collections::dataset as ds;

#[test]
fn canonical_beginning_modules_export_expected_types() {
    let _: Option<ds::frame::DatasetDataFrameNameSpace> = None;
    let _: Option<ds::series::DatasetSeriesNameSpace> = None;
    let _: Option<ds::expr::DatasetExprNameSpace> = None;
}

#[test]
fn canonical_essence_modules_export_expected_types() {
    let _: Option<ds::model::prep::ModelEssence> = None;
    let _: Option<ds::model::exec::Execution> = None;
    let _: Option<ds::model::image::ImageOptions> = None;
    let _: Option<ds::feature::featstruct::FeatStruct> = None;
    let _: Option<ds::feature::role::FeatureFrame> = None;
}

#[test]
fn canonical_concept_return_exports_expected_types() {
    type DefaultLm = ds::lm::MLE;
    assert_eq!(
        TypeId::of::<ds::sem::SemDataset<DefaultLm>>(),
        TypeId::of::<ds::SemDataset<DefaultLm>>()
    );
    assert_eq!(TypeId::of::<ds::sem::SemForm>(), TypeId::of::<ds::SemForm>());
    assert_eq!(TypeId::of::<ds::sem::SemError>(), TypeId::of::<ds::SemError>());
}

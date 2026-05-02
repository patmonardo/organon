use std::any::TypeId;

use gds::collections::dataset as ds;

#[test]
fn canonical_beginning_modules_export_expected_types() {
    let _: Option<ds::frame::DatasetDataFrameNameSpace> = None;
    let _: Option<ds::series::DatasetSeriesNameSpace> = None;
    let _: Option<ds::expr::DatasetExprNameSpace> = None;
}

#[test]
fn canonical_and_shim_essence_paths_are_type_equivalent() {
    assert_eq!(
        TypeId::of::<ds::model::prep::ModelEssence>(),
        TypeId::of::<ds::model_prep::ModelEssence>()
    );
    assert_eq!(
        TypeId::of::<ds::model::exec::Execution>(),
        TypeId::of::<ds::model_exec::Execution>()
    );
    assert_eq!(
        TypeId::of::<ds::model::image::ImageOptions>(),
        TypeId::of::<ds::model_image::ImageOptions>()
    );
    assert_eq!(
        TypeId::of::<ds::feature::featstruct::FeatStruct>(),
        TypeId::of::<ds::featstruct::FeatStruct>()
    );
    assert_eq!(
        TypeId::of::<ds::feature::role::FeatureFrame>(),
        TypeId::of::<ds::feature_role::FeatureFrame>()
    );
}

#[test]
fn canonical_and_shim_concept_paths_are_type_equivalent() {
    type DefaultLm = ds::lm::MLE;
    assert_eq!(
        TypeId::of::<ds::sem::SemDataset<DefaultLm>>(),
        TypeId::of::<ds::semantic::SemDataset<DefaultLm>>()
    );
    assert_eq!(
        TypeId::of::<ds::sem::SemForm>(),
        TypeId::of::<ds::semantic::SemForm>()
    );
    assert_eq!(
        TypeId::of::<ds::sem::SemError>(),
        TypeId::of::<ds::semantic::SemError>()
    );
}

use tiff::{TiffError, TiffFormatError, TiffResult};

const MODEL_TIE_POINT_TAG: &str = "ModelTiePointTag";
const MODEL_PIXEL_SCALE_TAG: &str = "ModelPixelScaleTag";
const MODEL_TRANSFORMATION_TAG: &str = "ModelTransformationTag";

#[derive(Debug)]
pub(super) enum CoordinateTransform {
    TiePointAndPixelScale([f64; 6], [f64; 3]),
    TiePoints(Vec<f64>),
    AffineTransform([f64; 16]),
}

impl CoordinateTransform {
    pub(super) fn from_tag_data(
        pixel_scale_data: Option<Vec<f64>>,
        model_tie_points_data: Option<Vec<f64>>,
        model_transformation_data: Option<Vec<f64>>,
    ) -> TiffResult<Self> {
        let pixel_scale = pixel_scale_data
            .map(|data| {
                <[f64; 3]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number values in {MODEL_PIXEL_SCALE_TAG} must be equal to 3"
                    )))
                })
            })
            .transpose()?;
        let tie_points = model_tie_points_data
            .map(|data| {
                let len = data.len();
                if len == 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be greater than 0"
                    ))));
                }

                if len % 6 != 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be divisible by 6"
                    ))));
                }

                Ok(data)
            })
            .transpose()?;
        let transformation = model_transformation_data
            .map(|data| {
                <[f64; 16]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TRANSFORMATION_TAG} must be equal to 16"
                    )))
                })
            })
            .transpose()?;

        if let Some(transformation) = transformation {
            if pixel_scale.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_PIXEL_SCALE_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }
            if tie_points.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }
            Ok(CoordinateTransform::AffineTransform(transformation))
        } else {
            let Some(tie_points) = tie_points else {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must be present when {MODEL_TRANSFORMATION_TAG} is missing"),
                )));
            };

            if tie_points.len() == 6 {
                let tie_point = <[f64; 6]>::try_from(tie_points).unwrap();
                let Some(pixel_scale) = pixel_scale else {
                    return Err(TiffError::FormatError(TiffFormatError::Format(
                        format!("{MODEL_PIXEL_SCALE_TAG} must be specified when {MODEL_TIE_POINT_TAG} contains 6 values"),
                    )));
                };
                Ok(CoordinateTransform::TiePointAndPixelScale(
                    tie_point,
                    pixel_scale,
                ))
            } else {
                Ok(CoordinateTransform::TiePoints(tie_points))
            }
        }
    }

    pub(super) fn transform_to_model(&self, coordinate: [usize; 2]) -> [f64; 2] {
        match self {
            CoordinateTransform::TiePointAndPixelScale(tie_point, pixel_scale) => {
                Self::transform_to_model_by_tie_point_and_pixel_scale(
                    tie_point,
                    pixel_scale,
                    coordinate,
                )
            }
            CoordinateTransform::TiePoints(tie_points) => {
                Self::transform_to_model_by_tie_points(tie_points, coordinate)
            }
            CoordinateTransform::AffineTransform(transformation_matrix) => {
                Self::transform_to_model_by_transformation_matrix(transformation_matrix, coordinate)
            }
        }
    }

    pub(super) fn transform_to_raster(&self, coordinate: &[f64; 2]) -> [usize; 2] {
        match self {
            CoordinateTransform::TiePointAndPixelScale(tie_point, pixel_scale) => {
                Self::transform_to_raster_by_tie_point_and_pixel_scale(
                    tie_point,
                    pixel_scale,
                    coordinate,
                )
            }
            CoordinateTransform::TiePoints(tie_points) => {
                Self::transform_to_raster_by_tie_points(tie_points, coordinate)
            }
            CoordinateTransform::AffineTransform(transformation_matrix) => {
                Self::transform_to_raster_by_affine_transform(transformation_matrix, coordinate)
            }
        }
    }

    fn transform_to_model_by_tie_point_and_pixel_scale(
        _tie_point: &[f64; 6],
        _pixel_scale: &[f64; 3],
        _coordinate: [usize; 2],
    ) -> [f64; 2] {
        todo!()
    }

    fn transform_to_model_by_tie_points(
        _tie_points: &[f64],
        _coordinate: [usize; 2],
    ) -> [f64; 2] {
        todo!()
    }

    fn transform_to_model_by_transformation_matrix(
        _transformation_matrix: &[f64; 16],
        _coordinate: [usize; 2],
    ) -> [f64; 2] {
        todo!()
    }

    fn transform_to_raster_by_tie_point_and_pixel_scale(
        _tie_point: &[f64; 6],
        _pixel_scale: &[f64; 3],
        _coordinate: &[f64; 2],
    ) -> [usize; 2] {
        todo!()
    }

    fn transform_to_raster_by_tie_points(
        _tie_points: &[f64],
        _coordinate: &[f64; 2],
    ) -> [usize; 2] {
        todo!()
    }

    fn transform_to_raster_by_affine_transform(
        _transformation_matrix: &[f64; 16],
        _coordinate: &[f64; 2],
    ) -> [usize; 2] {
        todo!()
    }
}

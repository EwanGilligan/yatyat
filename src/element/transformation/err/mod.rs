use snafu::Snafu;
#[derive(Debug, Snafu)]
pub enum TransformationError {
    #[snafu(display("Invalid image {:?} for degree {}", image, degree))]
    InvalidImage { degree: usize, image: Vec<usize> },
    #[snafu(display("Invalid point {} for degree {}", point, degree))]
    InvalidPoint { degree: usize, point: usize },
    #[snafu(display("Operation only defined for equal degree : {} != {}", degree1, degree2))]
    MismatchingDegree { degree1: usize, degree2: usize },
}

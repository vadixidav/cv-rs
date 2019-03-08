#include "xfeatures2d.hpp"
#include "utils.hpp"

cv::Ptr<cv::xfeatures2d::SURF>*
cv_surf_new(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright) {
    auto result = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
    return new cv::Ptr<cv::xfeatures2d::SURF>(result);
}
void cv_surf_drop(cv::Ptr<cv::xfeatures2d::SURF>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_surf_detect_and_compute(cv::Ptr<cv::xfeatures2d::SURF>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}

cv::Ptr<cv::xfeatures2d::SIFT>*
cv_sift_new(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    auto result = cv::xfeatures2d::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new cv::Ptr<cv::xfeatures2d::SIFT>(result);
}
void cv_sift_drop(cv::Ptr<cv::xfeatures2d::SIFT>* detector) {
    delete detector;
    detector = nullptr;
}

void cv_sift_detect_and_compute(cv::Ptr<cv::xfeatures2d::SIFT>* detector,
                                cv::Mat* image,
                                cv::Mat* mask,
                                CVec<KeyPoint>* keypoints,
                                cv::Mat* descriptors,
                                bool useProvidedKeypoints) {
    std::vector<cv::KeyPoint> keypoints_vector;
    detector->get()->detectAndCompute(*image, *mask, keypoints_vector, *descriptors, useProvidedKeypoints);
    cv_to_ffi(keypoints_vector, keypoints);
}
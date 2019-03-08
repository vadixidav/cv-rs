#ifndef HASH_H_
#define HASH_H_

#include <opencv2/core.hpp>
#include <opencv2/img_hash.hpp>

extern "C" {

void cv_hash_compute(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& mat, cv::Mat& result);
double cv_hash_compare(cv::Ptr<cv::img_hash::ImgHashBase>* phash, cv::Mat& lhs, cv::Mat& rhs);

cv::Ptr<cv::img_hash::AverageHash>* cv_average_hash_new();
void cv_average_hash_drop(cv::Ptr<cv::img_hash::AverageHash>* phash);

cv::Ptr<cv::img_hash::BlockMeanHash>* cv_block_mean_hash_new();
void cv_block_mean_hash_drop(cv::Ptr<cv::img_hash::BlockMeanHash>* phash);

cv::Ptr<cv::img_hash::ColorMomentHash>* cv_color_moment_hash_new();
void cv_color_moment_hash_drop(cv::Ptr<cv::img_hash::ColorMomentHash>* phash);

cv::Ptr<cv::img_hash::MarrHildrethHash>* cv_marr_hildreth_hash_new();
void cv_marr_hildreth_hash_drop(cv::Ptr<cv::img_hash::MarrHildrethHash>* phash);

cv::Ptr<cv::img_hash::PHash>* cv_phash_new();
void cv_phash_drop(cv::Ptr<cv::img_hash::PHash>* phash);

cv::Ptr<cv::img_hash::RadialVarianceHash>* cv_radial_variance_hash_new();
void cv_radial_variance_hash_drop(cv::Ptr<cv::img_hash::RadialVarianceHash>* phash);
}
#endif

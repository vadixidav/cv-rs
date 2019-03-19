extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use itertools::Itertools;
use std::env;
use std::path::PathBuf;

fn cmake_bool(flag: bool) -> &'static str {
    if flag {
        "ON"
    } else {
        "OFF"
    }
}

fn main() -> Result<(), std::io::Error> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let feature_cuda = env::var("CARGO_FEATURE_CUDA").is_ok();
    let feature_system = env::var("CARGO_FEATURE_SYSTEM").is_ok();

    // Get profile variables.
    let (configuration, lib_postfix) = match env::var("PROFILE").unwrap().as_str() {
        "debug" => ("Debug", if target_os == "windows" {"d"} else {""}),
        "release" => ("Release", ""),
        _ => panic!("unknown PROFILE env var from Cargo"),
    };

    let mut used_core_modules = vec![
        "core",
        "features2d",
        "flann",
        "highgui",
        "imgcodecs",
        "imgproc",
        "objdetect",
        "videoio",
        "video",
    ];

    // Add associated CUDA modules if the `cuda` feature is enabled.
    if feature_cuda {
        used_core_modules.push("cudaobjdetect");
    }

    // All the contrib modules used (core and contrib).
    let used_contrib_modules = vec!["xfeatures2d", "img_hash", "text"];

    // Collect all the modules used.
    let all_used_modules: Vec<_> = used_core_modules.iter().chain(used_contrib_modules.iter()).collect();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Path to contrib modules.
    let contrib_modules_path = manifest_dir.join("opencv_contrib").join("modules");
    // Global configuration for OpenCV build.
    let mut config = Config::new(".");

    // Set various cmake definitions.
    config
        // TODO: IPP creates some really annoying build issues on Windows.
        // Eventually we need to fix it.
        .define("CVSYS_SYSTEM_CV", cmake_bool(feature_system))
        .define("CV_CORE_MODULES", used_core_modules.join(";"))
        .define("CV_CONTRIB_MODULES", used_contrib_modules.join(";"))
        .define("WITH_IPP", "OFF")
        .define("BUILD_IPP_IW", "OFF")
        .define("BUILD_opencv_apps", "OFF")
        .define("BUILD_opencv_java_bindings_generator", "OFF")
        .define("BUILD_opencv_js", "OFF")
        .define("BUILD_opencv_python_bindings_generator", "OFF")
        .define("BUILD_opencv_world", "OFF")
        .define("INSTALL_CREATE_DISTRIB", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", configuration.to_ascii_uppercase())
        .define("INSTALL_PYTHON_EXAMPLES", "OFF")
        .define("INSTALL_C_EXAMPLES", "OFF")
        .define("OPENCV_ENABLE_NONFREE", "ON")
        .define("OPENCV_EXTRA_MODULES_PATH", contrib_modules_path.into_os_string())
        .define(
            "WITH_CUDA",
            cmake_bool(feature_cuda),
        )
        .define("BUILD_opencv_python", "OFF")
        .define("BUILD_opencv_python2", "OFF")
        .define("BUILD_opencv_python3", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_PERF_TESTS", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("BUILD_EXAMPLES", "OFF");

    if feature_system {
        let opencv_lib_dir = env::var("OPENCV_LIB").unwrap_or_else(|_| panic!("OPENCV_LIB not set (set it to where the OpenCV libs are)"));
        let opencv_dir = env::var("OPENCV_DIR").unwrap_or_else(|_| panic!("OPENCV_DIR not set (set it to the directory the include folder is in)"));
        let opencv_include_dir = PathBuf::from(opencv_dir).join("include");
        config
            .define("CVSYS_LIB_DIR", opencv_lib_dir)
            .define("CVSYS_INCLUDE_DIR", opencv_include_dir);
    }

    // Set which modules we want to build.
    for module in &all_used_modules {
        config.define(format!("BUILD_opencv_{}", module), "ON");
    }

    // Handle OS-specific requirements.
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=gomp");
            println!("cargo:rustc-link-lib=stdc++");

            config.define("ENABLE_PRECOMPILED_HEADERS", "OFF");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=comdlg32");
            println!("cargo:rustc-link-lib=Vfw32");
            println!("cargo:rustc-link-lib=Ole32");
            println!("cargo:rustc-link-lib=OleAut32");

            config.static_crt(true)
                .define("CMAKE_C_FLAGS", "/MT")
                .define("CMAKE_C_FLAGS_DEBUG", "/MT")
                .define("CMAKE_C_FLAGS_RELEASE", "/MT")
                .define("CMAKE_C_FLAGS_MINSIZEREL", "/MT")
                .define("CMAKE_C_FLAGS_RELWITHDEBINFO", "/MT")
                .define("CMAKE_CXX_FLAGS", "/MT")
                .define("CMAKE_CXX_FLAGS_DEBUG", "/MT")
                .define("CMAKE_CXX_FLAGS_RELEASE", "/MT")
                .define("CMAKE_CXX_FLAGS_MINSIZEREL", "/MT")
                .define("CMAKE_CXX_FLAGS_RELWITHDEBINFO", "/MT")
                .define("BUILD_WITH_STATIC_CRT", "ON");
        }
        _ => {}
    }

    // Build OpenCV and add it to cargo.
    let dst = config.build();

    // Link cvsys.
    println!("cargo:rustc-link-lib=static=cvsys");

    if feature_system {
        // Add cvsys path.
        if target_os == "windows" {
                println!(
                    "cargo:rustc-link-search={}",
                    dst.join("build").join(configuration).display()
                );
        } else {
            println!(
                "cargo:rustc-link-search={}",
                dst.join("build").display()
            );
        }
        // Add opencv libs path.
        let opencv_lib_dir = env::var("OPENCV_LIB").unwrap_or_else(|_| panic!("OPENCV_LIB not set (set it to where the OpenCV libs are)"));
        println!(
            "cargo:rustc-link-search={}",
            opencv_lib_dir
        );
        // Link all modules used.
        let opencv_vers = env::var("OPENCV_VERS").unwrap_or_else(|_| panic!("OPENCV_VERS not set (set it to the number appended to lib names, e.g. 341)"));
        for module in &all_used_modules {
            println!(
                "cargo:rustc-link-lib=static=opencv_{}{}",
                module,
                opencv_vers
            );
        }
    } else {
        // Load link_libs_list.txt to get list of all static libs to link to.
        // This is generated in the CMakeLists.txt.
        let libs_list: Vec<PathBuf> = std::fs::read_to_string(dst.join("build").join("link_libs_list.txt"))
            .expect("CMakeLists.txt didn't produce libs list")
            .replace("$(Configuration)", configuration)
            .split(';')
            .map(Into::into)
            .collect();

        // This contains the raw lib names.
        let deduped_lib_names: Vec<String> = libs_list
            .iter()
            .map(|p| p.file_stem().expect("expected only files in libs list").to_str().expect("OpenCV lib names must be valid UTF-8").to_owned())
            .unique()
            .collect();

        // This contains the lib search paths.
        let deduped_lib_search_paths: Vec<PathBuf> = libs_list
            .iter()
            .map(|p| p.parent().expect("expected libs to be in a directory").to_path_buf())
            .unique()
            .collect();

        // Change linker paths based on OS.
        match target_os.as_str() {
            "windows" => {
                // Add cvsys path.
                println!(
                    "cargo:rustc-link-search={}",
                    dst.join("build").join(configuration).display()
                );

                // Add all the static libs that need to be linked to Cargo, adding
                // postfixes as necessary (on Windows a 'd' is appended to libs in debug mode).
                for libname in deduped_lib_names {
                    println!(
                        "cargo:rustc-link-lib=static={}{}",
                        libname,
                        lib_postfix
                    );
                }
            }
            _ => {
                // Add cvsys path.
                println!(
                    "cargo:rustc-link-search={}",
                    dst.join("build").display()
                );

                // Add all the static libs that need to be linked to Cargo, adding
                // postfixes as necessary (on Windows a 'd' is appended to libs in debug mode).
                for libname in deduped_lib_names {
                    println!(
                        "cargo:rustc-link-lib=static={}",
                        &libname[3..]
                    );
                }
            },
        };

        // Add all cvsys dependencies' paths.
        for libpath in deduped_lib_search_paths {
            println!("cargo:rustc-link-search={}", libpath.display());
        }
    }

    // Set up bindgen to generate bindings from our C++ wrapper.
    // This whitelists exactly the stuff that is needed using regex.
    let bindings = Builder::default()
        .rustfmt_bindings(true)
        .whitelist_recursively(false)
        .derive_eq(true)
        .derive_ord(true)
        .derive_hash(true)
        .derive_debug(true)
        .derive_copy(true)
        .whitelist_function("cvsys::.*")
        .whitelist_type("cvsys::.*")
        .opaque_type("cv::.*")
        .opaque_type("std::.*")
        .whitelist_type("cv::MouseCallback")
        .whitelist_type("cv::text::classifier_type");
    
    // Change bindgen settings based on OS.
    let bindings = match target_os.as_str() {
        "linux" => {
            bindings.clang_args(&["-x", "c++", "-std=c++14", "-stdlib=libc++"])
        }
        _ => bindings,
    };

    let bindings = if feature_system {
        let opencv_dir = env::var("OPENCV_DIR").unwrap_or_else(|_| panic!("OPENCV_DIR not set (set it to the directory the include folder is in)"));
        let opencv_include_dir = PathBuf::from(opencv_dir).join("include");
        bindings.clang_arg(format!("-I{}", opencv_include_dir.display()))
    } else {
        // Add core module includes.
        let bindings = bindings.clang_args(used_core_modules.iter().map(|lib| {
            format!(
                "-I{}",
                manifest_dir
                    .join("opencv")
                    .join("modules")
                    .join(lib)
                    .join("include")
                    .display()
            )
        }));

        // Add contrib module includes.
        let bindings = bindings.clang_args(used_contrib_modules.iter().map(|lib| {
            format!(
                "-I{}",
                manifest_dir
                    .join("opencv_contrib")
                    .join("modules")
                    .join(lib)
                    .join("include")
                    .display()
            )
        }));

        // Add include from root of build folder.
        bindings.clang_arg(format!("-I{}", dst.join("include").display()))
    };

    // Add all wrapper headers.
    let bindings = all_used_modules
        .iter()
        .map(|lib| format!("native/{}.hpp", lib))
        .fold(bindings, Builder::header);

    // Finally generate the bindings.
    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings in the build directory.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

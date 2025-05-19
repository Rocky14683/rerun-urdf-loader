#include "ffi_binder/c_layer.hpp"
#include <iostream>
#include <string>
#include <print>

#include "urdf/common.h"
#include "urdf/model.h"

extern "C" {
void urdf_try() {
    std::println("hello from urdf_try cpp23 bind!");
}

URDFRobotHandle urdf_parse(const char *xml) {
    using namespace urdf;
    auto model = UrdfModel::fromUrdfStr(std::string(xml));

    assert(model && "Failed to parse URDF");
    assert(model->getLink("base_link") && "base_link not found");

    return new std::shared_ptr<UrdfModel>(model);
}

void urdf_free(URDFRobotHandle handle) {
    delete static_cast<std::shared_ptr<urdf::UrdfModel> *>(handle);
}

int urdf_visual_count(URDFRobotHandle handle) {
    auto model = *static_cast<std::shared_ptr<urdf::UrdfModel> *>(handle);
    auto base_link = model->getLink("base_link");
    assert(base_link && "base_link not found");
    return static_cast<int>(base_link->visuals.size());
}
} // extern C block
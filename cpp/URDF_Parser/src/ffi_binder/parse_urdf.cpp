#include "ffi_binder/c_layer.hpp"
#include <iostream>
#include <string>
#include <print>

#include "urdf/common.h"
#include "urdf/model.h"

#define GET_MODEL(handle) *static_cast<std::shared_ptr<urdf::UrdfModel> *>(handle)

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
    auto model = GET_MODEL(handle);
    auto base_link = model->getLink("base_link");
    assert(base_link && "base_link not found");
    return static_cast<int>(base_link->visuals.size());
}

const char* urdf_get_link_name(URDFRobotHandle handle, int index) {
    auto model = GET_MODEL(handle);
    auto base_link = model->getLink("base_link");
    assert(base_link && "base_link not found");
    assert(index >= 0 && index < base_link->visuals.size());

    //! this work because rust code will take the ownership of the pointer
    return base_link->name.c_str();
}

int urdf_get_geometry_type(URDFRobotHandle handle, int index) {
    auto model = GET_MODEL(handle);
    auto base_link = model->getLink("base_link");
    assert(base_link && "base_link not found");
    assert(index >= 0 && index < base_link->visuals.size());

    const auto& visual = base_link->visuals[index];
    if (visual->geometry.has_value()) {
        return static_cast<int>(visual->geometry.value()->type);
    }
    return -1;  // Or an error code
}


} // extern C block
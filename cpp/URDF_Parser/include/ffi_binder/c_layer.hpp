#pragma once

typedef void *URDFRobotHandle;


#ifdef __cplusplus
extern "C" {
#endif

void urdf_try();

URDFRobotHandle urdf_parse(const char *xml);
void urdf_free(URDFRobotHandle handle);
int urdf_visual_count(URDFRobotHandle handle);
const char* urdf_get_link_name(URDFRobotHandle handle, int index);
int urdf_get_geometry_type(URDFRobotHandle handle, int index);

#ifdef __cplusplus
}
#endif
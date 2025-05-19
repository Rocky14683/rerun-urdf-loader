#pragma once

typedef void *URDFRobotHandle;


#ifdef __cplusplus
extern "C" {
#endif

void urdf_try();

URDFRobotHandle urdf_parse(const char *xml);
void urdf_free(URDFRobotHandle robot);
int urdf_visual_count(URDFRobotHandle robot);

#ifdef __cplusplus
}
#endif
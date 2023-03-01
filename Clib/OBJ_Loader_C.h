#ifndef __LOADER_C_H__
#define __LOADER_C_H__

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void *create_new_loader();

void delete_loader(void *loader);

int load_file(void *loader, char *file);

void *loaded_meshes(void *loader, int *nmesh);

// Mesh*
void *mesh_at(void *meshes, size_t idx);

size_t vertex_size_mesh(void *mesh);

float *mesh_position_at(void *mesh, size_t idx);

float *mesh_normal_at(void *mesh, size_t idx);

float *mesh_texture_at(void *mesh, size_t idx);

#ifdef __cplusplus
}
#endif


#endif
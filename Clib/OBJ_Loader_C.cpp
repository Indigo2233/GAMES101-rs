#include "OBJ_Loader_C.h"
#include "OBJ_Loader.h"
/*
 * Building this C wrapper:
 * g++ -shared -o libobjloader.so OBJ_Loader_C.cpp -lc -fPIC
 */
extern "C" {
void *create_new_loader() {
    return new objl::Loader();
}

void delete_loader(void *loader) {
    objl::Loader *l = (objl::Loader *) loader;
    delete l;
}

int load_file(void *loader, char *file) {
    objl::Loader *l = (objl::Loader *) loader;
    return l->LoadFile(file);
}

void *loaded_meshes(void *loader, int *nmesh) {
    objl::Loader *l = (objl::Loader *) loader;
    *nmesh = l->LoadedMeshes.size();

    if (*nmesh != 0) return &l->LoadedMeshes;
    return NULL;
}

// Mesh*
void *mesh_at(void *meshes, size_t idx) {
    std::vector <objl::Mesh> *ms = (std::vector <objl::Mesh> *) meshes;
    return &(ms->at(idx));
}

size_t vertex_size_mesh(void *mesh) {
    objl::Mesh *m = (objl::Mesh *) mesh;
    return m->Vertices.size();
}

float *mesh_position_at(void *mesh, size_t idx) {
    objl::Mesh *m = (objl::Mesh *) mesh;
    objl::Vector3 position = m->Vertices[idx].Position;
    float *res = new float[3];
    res[0] = position.X;
    res[1] = position.Y;
    res[2] = position.Z;
    return res;
}

float* mesh_normal_at(void *mesh, size_t idx) {
    objl::Mesh *m = (objl::Mesh *) mesh;
    objl::Vector3 normal = m->Vertices[idx].Normal;
    float *res = new float[3];
    res[0] = normal.X;
    res[1] = normal.Y;
    res[2] = normal.Z;
    return res;
}

float* mesh_texture_at(void *mesh, size_t idx) {
    objl::Mesh *m = (objl::Mesh *) mesh;
    objl::Vector2 texture = m->Vertices[idx].TextureCoordinate;
    float *res = new float[2];
    res[0] = texture.X;
    res[1] = texture.Y;
    return res;
}
}
#include <vector>
#include <stdint.h>

class Vertex {
private:
public:
    uint32_t member_of;
    uint32_t current_degree;
};

class ConnectionComponent {
private:
public:
    uint32_t id;
};

class Solution {
private:
public:
    uint32_t cost;
    uint32_t number_connection_components;
    std::vector<Vertex> vertices;
    std::vector<ConnectionComponent> connectionComponents;
};

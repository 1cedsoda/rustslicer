#!/usr/bin/env python3
"""Generate a simple 20mm calibration cube STL file"""

import struct

def write_stl_triangle(f, normal, v1, v2, v3):
    """Write a single triangle to binary STL"""
    f.write(struct.pack('<3f', *normal))
    f.write(struct.pack('<3f', *v1))
    f.write(struct.pack('<3f', *v2))
    f.write(struct.pack('<3f', *v3))
    f.write(struct.pack('<H', 0))  # attribute byte count

def generate_cube_stl(filename, size=20.0):
    """Generate a cube STL file centered at origin"""
    half = size / 2.0
    
    # Define 8 vertices of the cube
    vertices = [
        (-half, -half, 0),      # 0: bottom-front-left
        ( half, -half, 0),      # 1: bottom-front-right
        ( half,  half, 0),      # 2: bottom-back-right
        (-half,  half, 0),      # 3: bottom-back-left
        (-half, -half, size),   # 4: top-front-left
        ( half, -half, size),   # 5: top-front-right
        ( half,  half, size),   # 6: top-back-right
        (-half,  half, size),   # 7: top-back-left
    ]
    
    # Define 12 triangles (2 per face, 6 faces)
    triangles = [
        # Bottom face (z=0) - normal pointing down
        ((0, 0, -1), vertices[0], vertices[2], vertices[1]),
        ((0, 0, -1), vertices[0], vertices[3], vertices[2]),
        
        # Top face (z=size) - normal pointing up
        ((0, 0, 1), vertices[4], vertices[5], vertices[6]),
        ((0, 0, 1), vertices[4], vertices[6], vertices[7]),
        
        # Front face (y=-half) - normal pointing forward
        ((0, -1, 0), vertices[0], vertices[1], vertices[5]),
        ((0, -1, 0), vertices[0], vertices[5], vertices[4]),
        
        # Back face (y=half) - normal pointing back
        ((0, 1, 0), vertices[3], vertices[6], vertices[2]),
        ((0, 1, 0), vertices[3], vertices[7], vertices[6]),
        
        # Left face (x=-half) - normal pointing left
        ((-1, 0, 0), vertices[0], vertices[4], vertices[7]),
        ((-1, 0, 0), vertices[0], vertices[7], vertices[3]),
        
        # Right face (x=half) - normal pointing right
        ((1, 0, 0), vertices[1], vertices[2], vertices[6]),
        ((1, 0, 0), vertices[1], vertices[6], vertices[5]),
    ]
    
    with open(filename, 'wb') as f:
        # Write header (80 bytes)
        header = b'Binary STL - 20mm Calibration Cube'
        f.write(header.ljust(80, b'\0'))
        
        # Write number of triangles
        f.write(struct.pack('<I', len(triangles)))
        
        # Write all triangles
        for normal, v1, v2, v3 in triangles:
            write_stl_triangle(f, normal, v1, v2, v3)
    
    print(f"Generated {filename} with {len(triangles)} triangles")

if __name__ == '__main__':
    generate_cube_stl('calibration_cube.stl', 20.0)

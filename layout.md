Layout for base renderer

meshes layout

```
buffer mesh {
    model0[px0,py0,pz0;uvx0,uvy0;nx0,ny0,nz0;..;pxn,pyn,pzn;uvxn,uvyn;nxn,nyn,nzn;],model1[0..n] 
}

buffer index{
    model0[inicies],model1[indicies+(size-1) + (size-2)..+(size-n)]
}
```

```
open gl

Vertex {
         (vao layout / shader layout)
         //mesh level
         (0) in vector3 position
(#ifdef) (1) in vector2 uv  //optional
         (2) in vector3 normals
         //instance level
         (3) in float32 material_id / shader_flow_id
      (4..7) in matrix4 model
     (8..11) in matrix4 mvp
    (11..16) in nothing
         (shader layout)
         //material data storage
         block material_data { // ubo /ssbo
            
         } data_partition[n]
         //global
         block matrices { //ubo
             uniform vp
             uniform p
             uniform v
         }
         block light{ // ubo/ssbo
            uniform LigthType1 ligth_1[n_light]
            uniform LigthType2 ligth_2[n_light]
            uniform LigthType3 ligth_3[n_light]
         }
}

```
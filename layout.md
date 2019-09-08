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
api

```

let cmd_buffer = device.create_comand_buffer();
let render_pass = device.create_render_pass(RenderPassDesc {
    attachemets_info(color, stencil, etc)
})
device.create_framebuffer(render_pass?, dimensions)
let pipeline = device.create_pipeline(PipelineDesc {
    buffer_binding_desc + attribs_desc + buffer,
    shaders_set, pipeline_layout(uniforms)
    blending_info,
    primitives_to_use(triangles, fan, etc)
})

cmd_buffer.viewport(dimensions)
cmd_buffer.prepare_pipeline(pipeline)
cmd_buffer.bind_vertex_buffer(0, vb)
cmd_buffer.bind_index_buffer(ib)

let shit = cmd_buffer.prepare_render_pass(rende_pass, framebuffer, viewport, clear_optional)

shit.draw* (instanced)
shit.draw_indexed* (instanced)
shit.draw_base_vertex (instanced
shit.draw_indexed_base_vertex (instanced)
cmd_buffer.finish()

device.submit(cmd_buffer)



```
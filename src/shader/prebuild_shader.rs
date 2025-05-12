use super::{Shader, ShaderProgram};

pub trait PrebuildShaderProgram {
    fn build_shader_program() -> ShaderProgram;
}

pub struct Texture2DShader ;
impl PrebuildShaderProgram for Texture2DShader  {
    fn build_shader_program() -> ShaderProgram {

        let vertex_shader = Shader {
            source: String::from("
                #version 410 core
                layout(location = 0) in vec3 inPosition;
                layout(location = 1) in vec2 inTexCoord;

                out vec2 texCoord;

                uniform mat4 p_mat;
                uniform mat4 v_mat;
                uniform mat4 m_mat;
                
                void main() {
                    mat4 mvp = p_mat * v_mat * m_mat;
                    gl_Position = mvp * vec4(inPosition, 1.0);
                    texCoord = inTexCoord;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 330 core
                in vec2 texCoord;
                in vec4 vColor;

                out vec4 fragColor; 

                uniform vec4 vertexColor;
                uniform vec4 uvTransform;
                uniform vec2 uvScale;
                uniform sampler2D textureSampler;

                void main() {
                    vec2 localUV = fract(texCoord * uvScale);
                    vec2 transformedTexCoord = localUV * uvTransform.xy + uvTransform.zw;
                    vec4 texColor = texture(textureSampler, transformedTexCoord);

                    fragColor = texColor * vertexColor;
                }
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }
}

pub struct ScreenShader;
impl PrebuildShaderProgram for ScreenShader {

    fn build_shader_program() -> ShaderProgram {

        let vertex_shader = Shader {
            source: String::from("
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec2 aTexCoords;

                out vec2 TexCoords;

                void main()
                {
                    gl_Position = vec4(aPos, 1.0); 
                    TexCoords = aTexCoords;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 330 core
                out vec4 FragColor;
  
                in vec2 TexCoords;

                uniform sampler2D textureSampler;
                //uniform float gamma;

                void main()
                { 
                    vec4 texColor = texture(textureSampler, TexCoords);
                    //texColor.rgb = pow(texColor.rgb, vec3(1.0 / gamma));
                    FragColor = texColor;
                }
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }

}

pub struct Texture2DBatchShader;
impl PrebuildShaderProgram for Texture2DBatchShader {

    fn build_shader_program() -> ShaderProgram {

        let vertex_shader = Shader {
            source: String::from("
                #version 330 core
                layout(location = 0) in vec3 inPosition;
                layout(location = 1) in vec2 inTexCoord;
                layout(location = 2) in vec4 inInstanceVertexColor;
                layout(location = 3) in mat4 inInstanceMatrix;
                layout(location = 7) in vec4 inUvTransform;
                layout(location = 8) in vec4 inExtras;

                uniform mat4 p_mat;
                uniform mat4 v_mat;

                out vec3 fragPos;
                out vec2 texCoord;
                out vec4 vertexColor;
                out vec4 uvTransform;
                out vec4 extras;

                void main() {
                    mat4 mvp = p_mat * v_mat * inInstanceMatrix;
                    gl_Position = mvp * vec4(inPosition, 1.0);
                    fragPos = inPosition;
                    texCoord = inTexCoord;
                    vertexColor = inInstanceVertexColor;
                    uvTransform = inUvTransform;
                    extras = inExtras;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 330 core

                in vec3 fragPos;
                in vec2 texCoord;
                in vec4 vertexColor;
                in vec4 uvTransform;
                in vec4 extras;

                out vec4 FragColor;
                
                uniform sampler2D textureSampler;

                void main() {
                    if(extras.x == 0.0) {
                        discard;
                    }
                    vec2 transformedTexCoord = texCoord * uvTransform.xy + uvTransform.zw;
                    vec4 texColor = texture(textureSampler, transformedTexCoord);
                    FragColor = texColor * vertexColor;
                }
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }

}


pub struct FontShader;
impl PrebuildShaderProgram for FontShader {

    fn build_shader_program() -> ShaderProgram {

        let vertex_shader = Shader {
            source: String::from("
                #version 330 core
                layout (location = 0) in vec4 vertex;
                out vec2 TexCoords;

                uniform mat4 p_mat;

                void main()
                {
                    gl_Position = p_mat * vec4(vertex.xy, 0.0, 1.0);
                    TexCoords = vertex.zw;
                }  
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 330 core
                in vec2 TexCoords;
                out vec4 color;

                uniform sampler2D textureSampler;
                uniform vec4 vertexColor;

                void main()
                {    
                    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(textureSampler, TexCoords).r);
                    color = vertexColor * sampled;
                }  
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }

}

pub struct RectShader;
impl PrebuildShaderProgram for RectShader {

    fn build_shader_program() -> ShaderProgram {
        
        let vertex_shader = Shader {
            source: String::from("
                #version 410 core
                layout(location = 0) in vec3 inPosition;

                out vec3 position;

                uniform mat4 p_mat;
                uniform mat4 v_mat;
                uniform mat4 m_mat;
                
                void main() {
                    mat4 mvp = p_mat * v_mat * m_mat;
                    gl_Position = mvp * vec4(inPosition, 1.0);
                    position = inPosition;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 410 core

                out vec4 fragColor; 
                in vec3 position;

                uniform vec4 vertexColor;
                uniform float aspect;
                uniform float borderWidth;
                uniform bool isSolid;

                void main() {
                    if(isSolid) {
                        fragColor = vertexColor;
                        return;
                    }

                    float bw = (borderWidth / 100) * aspect;
                    float maxX = 0.5 - bw / aspect;
                    float minX = -0.5 + bw / aspect;
                    float maxY = 0.5 - bw;
                    float minY = -0.5 + bw;

                   if (position.x < maxX && position.x > minX && position.y < maxY && position.y > minY) {
                        discard;
                   } else {
                        fragColor = vertexColor;
                   }  
                }
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }
}

pub struct MeshShader;
impl PrebuildShaderProgram for MeshShader {

    fn build_shader_program() -> ShaderProgram {
        
        let vertex_shader = Shader {
            source: String::from("
                #version 410 core
                layout(location = 0) in vec3 inPosition;
                layout(location = 1) in vec2 inTexCoord;
                layout(location = 2) in vec3 inNormal;

                out vec3 position;
                out vec3 normal;
                out vec2 texCoord;

                uniform mat4 p_mat;
                uniform mat4 v_mat;
                uniform mat4 m_mat;
                
                void main() {
                    mat4 mvp = p_mat * v_mat * m_mat;
                    gl_Position = mvp * vec4(inPosition, 1.0);
                    position = inPosition;
                    normal = inNormal;
                    texCoord = inTexCoord;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 410 core

                out vec4 FragColor; 
                
                in vec3 position;
                in vec3 normal;
                in vec2 texCoord;

                uniform vec4 vertexColor;
                uniform sampler2D textureSampler;

                void main()
                { 
                    vec4 texColor = texture(textureSampler, texCoord);
                    //texColor.rgb = pow(texColor.rgb, vec3(1.0 / gamma));
                    FragColor = texColor;
                }
            ")
        };

        return ShaderProgram::PreBuild {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader
        }
    }
}
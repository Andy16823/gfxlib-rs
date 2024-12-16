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
                out vec4 vColor;

                uniform mat4 p_mat;
                uniform mat4 v_mat;
                uniform mat4 m_mat;
                uniform vec4 vertexColor;

                void main() {
                    mat4 mvp = p_mat * v_mat * m_mat;
                    gl_Position = mvp * vec4(inPosition, 1.0);
                    texCoord = inTexCoord;
                    vColor = vertexColor;
                }
            "),
        };

        let fragment_shader = Shader {
            source: String::from("
                #version 330 core
                in vec2 texCoord;
                in vec4 vColor;

                out vec4 fragColor; 

                uniform sampler2D textureSampler;

                void main() {
                    fragColor = texture(textureSampler, texCoord) * vColor;
                }
            ")
        };

        return ShaderProgram {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program_id: 0
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

        return ShaderProgram {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program_id: 0
        }
    }

}

// From vector 32 to 255 create a handler
.code64

.text

.extern save_regs
.extern restore_regs
.extern isr_handler

.global iv_32
iv_32:
    call save_regs
    mov rdi, 32
    call isr_handler
    call restore_regs
    iretq

.global iv_33
iv_33:
    call save_regs
    mov rdi, 33
    call isr_handler
    call restore_regs
    iretq

.global iv_34
iv_34:
    call save_regs
    mov rdi, 34
    call isr_handler
    call restore_regs
    iretq

.global iv_35
iv_35:
    call save_regs
    mov rdi, 35
    call isr_handler
    call restore_regs
    iretq

.global iv_36
iv_36:
    call save_regs
    mov rdi, 36
    call isr_handler
    call restore_regs
    iretq

.global iv_37
iv_37:
    call save_regs
    mov rdi, 37
    call isr_handler
    call restore_regs
    iretq

.global iv_38
iv_38:
    call save_regs
    mov rdi, 38
    call isr_handler
    call restore_regs
    iretq

.global iv_39
iv_39:
    call save_regs
    mov rdi, 39
    call isr_handler
    call restore_regs
    iretq

.global iv_40
iv_40:
    call save_regs
    mov rdi, 40
    call isr_handler
    call restore_regs
    iretq

.global iv_41
iv_41:
    call save_regs
    mov rdi, 41
    call isr_handler
    call restore_regs
    iretq

.global iv_42
iv_42:
    call save_regs
    mov rdi, 42
    call isr_handler
    call restore_regs
    iretq

.global iv_43
iv_43:
    call save_regs
    mov rdi, 43
    call isr_handler
    call restore_regs
    iretq

.global iv_44
iv_44:
    call save_regs
    mov rdi, 44
    call isr_handler
    call restore_regs
    iretq

.global iv_45
iv_45:
    call save_regs
    mov rdi, 45
    call isr_handler
    call restore_regs
    iretq

.global iv_46
iv_46:
    call save_regs
    mov rdi, 46
    call isr_handler
    call restore_regs
    iretq

.global iv_47
iv_47:
    call save_regs
    mov rdi, 47
    call isr_handler
    call restore_regs
    iretq

.global iv_48
iv_48:
    call save_regs
    mov rdi, 48
    call isr_handler
    call restore_regs
    iretq

.global iv_49
iv_49:
    call save_regs
    mov rdi, 49
    call isr_handler
    call restore_regs
    iretq

.global iv_50
iv_50:
    call save_regs
    mov rdi, 50
    call isr_handler
    call restore_regs
    iretq

.global iv_51
iv_51:
    call save_regs
    mov rdi, 51
    call isr_handler
    call restore_regs
    iretq

.global iv_52
iv_52:
    call save_regs
    mov rdi, 52
    call isr_handler
    call restore_regs
    iretq

.global iv_53
iv_53:
    call save_regs
    mov rdi, 53
    call isr_handler
    call restore_regs
    iretq

.global iv_54
iv_54:
    call save_regs
    mov rdi, 54
    call isr_handler
    call restore_regs
    iretq

.global iv_55
iv_55:
    call save_regs
    mov rdi, 55
    call isr_handler
    call restore_regs
    iretq

.global iv_56
iv_56:
    call save_regs
    mov rdi, 56
    call isr_handler
    call restore_regs
    iretq

.global iv_57
iv_57:
    call save_regs
    mov rdi, 57
    call isr_handler
    call restore_regs
    iretq

.global iv_58
iv_58:
    call save_regs
    mov rdi, 58
    call isr_handler
    call restore_regs
    iretq

.global iv_59
iv_59:
    call save_regs
    mov rdi, 59
    call isr_handler
    call restore_regs
    iretq

.global iv_60
iv_60:
    call save_regs
    mov rdi, 60
    call isr_handler
    call restore_regs
    iretq

.global iv_61
iv_61:
    call save_regs
    mov rdi, 61
    call isr_handler
    call restore_regs
    iretq

.global iv_62
iv_62:
    call save_regs
    mov rdi, 62
    call isr_handler
    call restore_regs
    iretq

.global iv_63
iv_63:
    call save_regs
    mov rdi, 63
    call isr_handler
    call restore_regs
    iretq

.global iv_64
iv_64:
    call save_regs
    mov rdi, 64
    call isr_handler
    call restore_regs
    iretq

.global iv_65
iv_65:
    call save_regs
    mov rdi, 65
    call isr_handler
    call restore_regs
    iretq

.global iv_66
iv_66:
    call save_regs
    mov rdi, 66
    call isr_handler
    call restore_regs
    iretq

.global iv_67
iv_67:
    call save_regs
    mov rdi, 67
    call isr_handler
    call restore_regs
    iretq

.global iv_68
iv_68:
    call save_regs
    mov rdi, 68
    call isr_handler
    call restore_regs
    iretq

.global iv_69
iv_69:
    call save_regs
    mov rdi, 69
    call isr_handler
    call restore_regs
    iretq

.global iv_70
iv_70:
    call save_regs
    mov rdi, 70
    call isr_handler
    call restore_regs
    iretq

.global iv_71
iv_71:
    call save_regs
    mov rdi, 71
    call isr_handler
    call restore_regs
    iretq

.global iv_72
iv_72:
    call save_regs
    mov rdi, 72
    call isr_handler
    call restore_regs
    iretq

.global iv_73
iv_73:
    call save_regs
    mov rdi, 73
    call isr_handler
    call restore_regs
    iretq

.global iv_74
iv_74:
    call save_regs
    mov rdi, 74
    call isr_handler
    call restore_regs
    iretq

.global iv_75
iv_75:
    call save_regs
    mov rdi, 75
    call isr_handler
    call restore_regs
    iretq

.global iv_76
iv_76:
    call save_regs
    mov rdi, 76
    call isr_handler
    call restore_regs
    iretq

.global iv_77
iv_77:
    call save_regs
    mov rdi, 77
    call isr_handler
    call restore_regs
    iretq

.global iv_78
iv_78:
    call save_regs
    mov rdi, 78
    call isr_handler
    call restore_regs
    iretq

.global iv_79
iv_79:
    call save_regs
    mov rdi, 79
    call isr_handler
    call restore_regs
    iretq

.global iv_80
iv_80:
    call save_regs
    mov rdi, 80
    call isr_handler
    call restore_regs
    iretq

.global iv_81
iv_81:
    call save_regs
    mov rdi, 81
    call isr_handler
    call restore_regs
    iretq

.global iv_82
iv_82:
    call save_regs
    mov rdi, 82
    call isr_handler
    call restore_regs
    iretq

.global iv_83
iv_83:
    call save_regs
    mov rdi, 83
    call isr_handler
    call restore_regs
    iretq

.global iv_84
iv_84:
    call save_regs
    mov rdi, 84
    call isr_handler
    call restore_regs
    iretq

.global iv_85
iv_85:
    call save_regs
    mov rdi, 85
    call isr_handler
    call restore_regs
    iretq

.global iv_86
iv_86:
    call save_regs
    mov rdi, 86
    call isr_handler
    call restore_regs
    iretq

.global iv_87
iv_87:
    call save_regs
    mov rdi, 87
    call isr_handler
    call restore_regs
    iretq

.global iv_88
iv_88:
    call save_regs
    mov rdi, 88
    call isr_handler
    call restore_regs
    iretq

.global iv_89
iv_89:
    call save_regs
    mov rdi, 89
    call isr_handler
    call restore_regs
    iretq

.global iv_90
iv_90:
    call save_regs
    mov rdi, 90
    call isr_handler
    call restore_regs
    iretq

.global iv_91
iv_91:
    call save_regs
    mov rdi, 91
    call isr_handler
    call restore_regs
    iretq

.global iv_92
iv_92:
    call save_regs
    mov rdi, 92
    call isr_handler
    call restore_regs
    iretq

.global iv_93
iv_93:
    call save_regs
    mov rdi, 93
    call isr_handler
    call restore_regs
    iretq

.global iv_94
iv_94:
    call save_regs
    mov rdi, 94
    call isr_handler
    call restore_regs
    iretq

.global iv_95
iv_95:
    call save_regs
    mov rdi, 95
    call isr_handler
    call restore_regs
    iretq

.global iv_96
iv_96:
    call save_regs
    mov rdi, 96
    call isr_handler
    call restore_regs
    iretq

.global iv_97
iv_97:
    call save_regs
    mov rdi, 97
    call isr_handler
    call restore_regs
    iretq

.global iv_98
iv_98:
    call save_regs
    mov rdi, 98
    call isr_handler
    call restore_regs
    iretq

.global iv_99
iv_99:
    call save_regs
    mov rdi, 99
    call isr_handler
    call restore_regs
    iretq

.global iv_100
iv_100:
    call save_regs
    mov rdi, 100
    call isr_handler
    call restore_regs
    iretq

.global iv_101
iv_101:
    call save_regs
    mov rdi, 101
    call isr_handler
    call restore_regs
    iretq

.global iv_102
iv_102:
    call save_regs
    mov rdi, 102
    call isr_handler
    call restore_regs
    iretq

.global iv_103
iv_103:
    call save_regs
    mov rdi, 103
    call isr_handler
    call restore_regs
    iretq

.global iv_104
iv_104:
    call save_regs
    mov rdi, 104
    call isr_handler
    call restore_regs
    iretq

.global iv_105
iv_105:
    call save_regs
    mov rdi, 105
    call isr_handler
    call restore_regs
    iretq

.global iv_106
iv_106:
    call save_regs
    mov rdi, 106
    call isr_handler
    call restore_regs
    iretq

.global iv_107
iv_107:
    call save_regs
    mov rdi, 107
    call isr_handler
    call restore_regs
    iretq

.global iv_108
iv_108:
    call save_regs
    mov rdi, 108
    call isr_handler
    call restore_regs
    iretq

.global iv_109
iv_109:
    call save_regs
    mov rdi, 109
    call isr_handler
    call restore_regs
    iretq

.global iv_110
iv_110:
    call save_regs
    mov rdi, 110
    call isr_handler
    call restore_regs
    iretq

.global iv_111
iv_111:
    call save_regs
    mov rdi, 111
    call isr_handler
    call restore_regs
    iretq

.global iv_112
iv_112:
    call save_regs
    mov rdi, 112
    call isr_handler
    call restore_regs
    iretq

.global iv_113
iv_113:
    call save_regs
    mov rdi, 113
    call isr_handler
    call restore_regs
    iretq

.global iv_114
iv_114:
    call save_regs
    mov rdi, 114
    call isr_handler
    call restore_regs
    iretq

.global iv_115
iv_115:
    call save_regs
    mov rdi, 115
    call isr_handler
    call restore_regs
    iretq

.global iv_116
iv_116:
    call save_regs
    mov rdi, 116
    call isr_handler
    call restore_regs
    iretq

.global iv_117
iv_117:
    call save_regs
    mov rdi, 117
    call isr_handler
    call restore_regs
    iretq

.global iv_118
iv_118:
    call save_regs
    mov rdi, 118
    call isr_handler
    call restore_regs
    iretq

.global iv_119
iv_119:
    call save_regs
    mov rdi, 119
    call isr_handler
    call restore_regs
    iretq

.global iv_120
iv_120:
    call save_regs
    mov rdi, 120
    call isr_handler
    call restore_regs
    iretq

.global iv_121
iv_121:
    call save_regs
    mov rdi, 121
    call isr_handler
    call restore_regs
    iretq

.global iv_122
iv_122:
    call save_regs
    mov rdi, 122
    call isr_handler
    call restore_regs
    iretq

.global iv_123
iv_123:
    call save_regs
    mov rdi, 123
    call isr_handler
    call restore_regs
    iretq

.global iv_124
iv_124:
    call save_regs
    mov rdi, 124
    call isr_handler
    call restore_regs
    iretq

.global iv_125
iv_125:
    call save_regs
    mov rdi, 125
    call isr_handler
    call restore_regs
    iretq

.global iv_126
iv_126:
    call save_regs
    mov rdi, 126
    call isr_handler
    call restore_regs
    iretq

.global iv_127
iv_127:
    call save_regs
    mov rdi, 127
    call isr_handler
    call restore_regs
    iretq

.global iv_128
iv_128:
    call save_regs
    mov rdi, 128
    call isr_handler
    call restore_regs
    iretq

.global iv_129
iv_129:
    call save_regs
    mov rdi, 129
    call isr_handler
    call restore_regs
    iretq

.global iv_130
iv_130:
    call save_regs
    mov rdi, 130
    call isr_handler
    call restore_regs
    iretq

.global iv_131
iv_131:
    call save_regs
    mov rdi, 131
    call isr_handler
    call restore_regs
    iretq

.global iv_132
iv_132:
    call save_regs
    mov rdi, 132
    call isr_handler
    call restore_regs
    iretq

.global iv_133
iv_133:
    call save_regs
    mov rdi, 133
    call isr_handler
    call restore_regs
    iretq

.global iv_134
iv_134:
    call save_regs
    mov rdi, 134
    call isr_handler
    call restore_regs
    iretq

.global iv_135
iv_135:
    call save_regs
    mov rdi, 135
    call isr_handler
    call restore_regs
    iretq

.global iv_136
iv_136:
    call save_regs
    mov rdi, 136
    call isr_handler
    call restore_regs
    iretq

.global iv_137
iv_137:
    call save_regs
    mov rdi, 137
    call isr_handler
    call restore_regs
    iretq

.global iv_138
iv_138:
    call save_regs
    mov rdi, 138
    call isr_handler
    call restore_regs
    iretq

.global iv_139
iv_139:
    call save_regs
    mov rdi, 139
    call isr_handler
    call restore_regs
    iretq

.global iv_140
iv_140:
    call save_regs
    mov rdi, 140
    call isr_handler
    call restore_regs
    iretq

.global iv_141
iv_141:
    call save_regs
    mov rdi, 141
    call isr_handler
    call restore_regs
    iretq

.global iv_142
iv_142:
    call save_regs
    mov rdi, 142
    call isr_handler
    call restore_regs
    iretq

.global iv_143
iv_143:
    call save_regs
    mov rdi, 143
    call isr_handler
    call restore_regs
    iretq

.global iv_144
iv_144:
    call save_regs
    mov rdi, 144
    call isr_handler
    call restore_regs
    iretq

.global iv_145
iv_145:
    call save_regs
    mov rdi, 145
    call isr_handler
    call restore_regs
    iretq

.global iv_146
iv_146:
    call save_regs
    mov rdi, 146
    call isr_handler
    call restore_regs
    iretq

.global iv_147
iv_147:
    call save_regs
    mov rdi, 147
    call isr_handler
    call restore_regs
    iretq

.global iv_148
iv_148:
    call save_regs
    mov rdi, 148
    call isr_handler
    call restore_regs
    iretq

.global iv_149
iv_149:
    call save_regs
    mov rdi, 149
    call isr_handler
    call restore_regs
    iretq

.global iv_150
iv_150:
    call save_regs
    mov rdi, 150
    call isr_handler
    call restore_regs
    iretq

.global iv_151
iv_151:
    call save_regs
    mov rdi, 151
    call isr_handler
    call restore_regs
    iretq

.global iv_152
iv_152:
    call save_regs
    mov rdi, 152
    call isr_handler
    call restore_regs
    iretq

.global iv_153
iv_153:
    call save_regs
    mov rdi, 153
    call isr_handler
    call restore_regs
    iretq

.global iv_154
iv_154:
    call save_regs
    mov rdi, 154
    call isr_handler
    call restore_regs
    iretq

.global iv_155
iv_155:
    call save_regs
    mov rdi, 155
    call isr_handler
    call restore_regs
    iretq

.global iv_156
iv_156:
    call save_regs
    mov rdi, 156
    call isr_handler
    call restore_regs
    iretq

.global iv_157
iv_157:
    call save_regs
    mov rdi, 157
    call isr_handler
    call restore_regs
    iretq

.global iv_158
iv_158:
    call save_regs
    mov rdi, 158
    call isr_handler
    call restore_regs
    iretq

.global iv_159
iv_159:
    call save_regs
    mov rdi, 159
    call isr_handler
    call restore_regs
    iretq

.global iv_160
iv_160:
    call save_regs
    mov rdi, 160
    call isr_handler
    call restore_regs
    iretq

.global iv_161
iv_161:
    call save_regs
    mov rdi, 161
    call isr_handler
    call restore_regs
    iretq

.global iv_162
iv_162:
    call save_regs
    mov rdi, 162
    call isr_handler
    call restore_regs
    iretq

.global iv_163
iv_163:
    call save_regs
    mov rdi, 163
    call isr_handler
    call restore_regs
    iretq

.global iv_164
iv_164:
    call save_regs
    mov rdi, 164
    call isr_handler
    call restore_regs
    iretq

.global iv_165
iv_165:
    call save_regs
    mov rdi, 165
    call isr_handler
    call restore_regs
    iretq

.global iv_166
iv_166:
    call save_regs
    mov rdi, 166
    call isr_handler
    call restore_regs
    iretq

.global iv_167
iv_167:
    call save_regs
    mov rdi, 167
    call isr_handler
    call restore_regs
    iretq

.global iv_168
iv_168:
    call save_regs
    mov rdi, 168
    call isr_handler
    call restore_regs
    iretq

.global iv_169
iv_169:
    call save_regs
    mov rdi, 169
    call isr_handler
    call restore_regs
    iretq

.global iv_170
iv_170:
    call save_regs
    mov rdi, 170
    call isr_handler
    call restore_regs
    iretq

.global iv_171
iv_171:
    call save_regs
    mov rdi, 171
    call isr_handler
    call restore_regs
    iretq

.global iv_172
iv_172:
    call save_regs
    mov rdi, 172
    call isr_handler
    call restore_regs
    iretq

.global iv_173
iv_173:
    call save_regs
    mov rdi, 173
    call isr_handler
    call restore_regs
    iretq

.global iv_174
iv_174:
    call save_regs
    mov rdi, 174
    call isr_handler
    call restore_regs
    iretq

.global iv_175
iv_175:
    call save_regs
    mov rdi, 175
    call isr_handler
    call restore_regs
    iretq

.global iv_176
iv_176:
    call save_regs
    mov rdi, 176
    call isr_handler
    call restore_regs
    iretq

.global iv_177
iv_177:
    call save_regs
    mov rdi, 177
    call isr_handler
    call restore_regs
    iretq

.global iv_178
iv_178:
    call save_regs
    mov rdi, 178
    call isr_handler
    call restore_regs
    iretq

.global iv_179
iv_179:
    call save_regs
    mov rdi, 179
    call isr_handler
    call restore_regs
    iretq

.global iv_180
iv_180:
    call save_regs
    mov rdi, 180
    call isr_handler
    call restore_regs
    iretq

.global iv_181
iv_181:
    call save_regs
    mov rdi, 181
    call isr_handler
    call restore_regs
    iretq

.global iv_182
iv_182:
    call save_regs
    mov rdi, 182
    call isr_handler
    call restore_regs
    iretq

.global iv_183
iv_183:
    call save_regs
    mov rdi, 183
    call isr_handler
    call restore_regs
    iretq

.global iv_184
iv_184:
    call save_regs
    mov rdi, 184
    call isr_handler
    call restore_regs
    iretq

.global iv_185
iv_185:
    call save_regs
    mov rdi, 185
    call isr_handler
    call restore_regs
    iretq

.global iv_186
iv_186:
    call save_regs
    mov rdi, 186
    call isr_handler
    call restore_regs
    iretq

.global iv_187
iv_187:
    call save_regs
    mov rdi, 187
    call isr_handler
    call restore_regs
    iretq

.global iv_188
iv_188:
    call save_regs
    mov rdi, 188
    call isr_handler
    call restore_regs
    iretq

.global iv_189
iv_189:
    call save_regs
    mov rdi, 189
    call isr_handler
    call restore_regs
    iretq

.global iv_190
iv_190:
    call save_regs
    mov rdi, 190
    call isr_handler
    call restore_regs
    iretq

.global iv_191
iv_191:
    call save_regs
    mov rdi, 191
    call isr_handler
    call restore_regs
    iretq

.global iv_192
iv_192:
    call save_regs
    mov rdi, 192
    call isr_handler
    call restore_regs
    iretq

.global iv_193
iv_193:
    call save_regs
    mov rdi, 193
    call isr_handler
    call restore_regs
    iretq

.global iv_194
iv_194:
    call save_regs
    mov rdi, 194
    call isr_handler
    call restore_regs
    iretq

.global iv_195
iv_195:
    call save_regs
    mov rdi, 195
    call isr_handler
    call restore_regs
    iretq

.global iv_196
iv_196:
    call save_regs
    mov rdi, 196
    call isr_handler
    call restore_regs
    iretq

.global iv_197
iv_197:
    call save_regs
    mov rdi, 197
    call isr_handler
    call restore_regs
    iretq

.global iv_198
iv_198:
    call save_regs
    mov rdi, 198
    call isr_handler
    call restore_regs
    iretq

.global iv_199
iv_199:
    call save_regs
    mov rdi, 199
    call isr_handler
    call restore_regs
    iretq

.global iv_200
iv_200:
    call save_regs
    mov rdi, 200
    call isr_handler
    call restore_regs
    iretq

.global iv_201
iv_201:
    call save_regs
    mov rdi, 201
    call isr_handler
    call restore_regs
    iretq

.global iv_202
iv_202:
    call save_regs
    mov rdi, 202
    call isr_handler
    call restore_regs
    iretq

.global iv_203
iv_203:
    call save_regs
    mov rdi, 203
    call isr_handler
    call restore_regs
    iretq

.global iv_204
iv_204:
    call save_regs
    mov rdi, 204
    call isr_handler
    call restore_regs
    iretq

.global iv_205
iv_205:
    call save_regs
    mov rdi, 205
    call isr_handler
    call restore_regs
    iretq

.global iv_206
iv_206:
    call save_regs
    mov rdi, 206
    call isr_handler
    call restore_regs
    iretq

.global iv_207
iv_207:
    call save_regs
    mov rdi, 207
    call isr_handler
    call restore_regs
    iretq

.global iv_208
iv_208:
    call save_regs
    mov rdi, 208
    call isr_handler
    call restore_regs
    iretq

.global iv_209
iv_209:
    call save_regs
    mov rdi, 209
    call isr_handler
    call restore_regs
    iretq

.global iv_210
iv_210:
    call save_regs
    mov rdi, 210
    call isr_handler
    call restore_regs
    iretq

.global iv_211
iv_211:
    call save_regs
    mov rdi, 211
    call isr_handler
    call restore_regs
    iretq

.global iv_212
iv_212:
    call save_regs
    mov rdi, 212
    call isr_handler
    call restore_regs
    iretq

.global iv_213
iv_213:
    call save_regs
    mov rdi, 213
    call isr_handler
    call restore_regs
    iretq

.global iv_214
iv_214:
    call save_regs
    mov rdi, 214
    call isr_handler
    call restore_regs
    iretq

.global iv_215
iv_215:
    call save_regs
    mov rdi, 215
    call isr_handler
    call restore_regs
    iretq

.global iv_216
iv_216:
    call save_regs
    mov rdi, 216
    call isr_handler
    call restore_regs
    iretq

.global iv_217
iv_217:
    call save_regs
    mov rdi, 217
    call isr_handler
    call restore_regs
    iretq

.global iv_218
iv_218:
    call save_regs
    mov rdi, 218
    call isr_handler
    call restore_regs
    iretq

.global iv_219
iv_219:
    call save_regs
    mov rdi, 219
    call isr_handler
    call restore_regs
    iretq

.global iv_220
iv_220:
    call save_regs
    mov rdi, 220
    call isr_handler
    call restore_regs
    iretq

.global iv_221
iv_221:
    call save_regs
    mov rdi, 221
    call isr_handler
    call restore_regs
    iretq

.global iv_222
iv_222:
    call save_regs
    mov rdi, 222
    call isr_handler
    call restore_regs
    iretq

.global iv_223
iv_223:
    call save_regs
    mov rdi, 223
    call isr_handler
    call restore_regs
    iretq

.global iv_224
iv_224:
    call save_regs
    mov rdi, 224
    call isr_handler
    call restore_regs
    iretq

.global iv_225
iv_225:
    call save_regs
    mov rdi, 225
    call isr_handler
    call restore_regs
    iretq

.global iv_226
iv_226:
    call save_regs
    mov rdi, 226
    call isr_handler
    call restore_regs
    iretq

.global iv_227
iv_227:
    call save_regs
    mov rdi, 227
    call isr_handler
    call restore_regs
    iretq

.global iv_228
iv_228:
    call save_regs
    mov rdi, 228
    call isr_handler
    call restore_regs
    iretq

.global iv_229
iv_229:
    call save_regs
    mov rdi, 229
    call isr_handler
    call restore_regs
    iretq

.global iv_230
iv_230:
    call save_regs
    mov rdi, 230
    call isr_handler
    call restore_regs
    iretq

.global iv_231
iv_231:
    call save_regs
    mov rdi, 231
    call isr_handler
    call restore_regs
    iretq

.global iv_232
iv_232:
    call save_regs
    mov rdi, 232
    call isr_handler
    call restore_regs
    iretq

.global iv_233
iv_233:
    call save_regs
    mov rdi, 233
    call isr_handler
    call restore_regs
    iretq

.global iv_234
iv_234:
    call save_regs
    mov rdi, 234
    call isr_handler
    call restore_regs
    iretq

.global iv_235
iv_235:
    call save_regs
    mov rdi, 235
    call isr_handler
    call restore_regs
    iretq

.global iv_236
iv_236:
    call save_regs
    mov rdi, 236
    call isr_handler
    call restore_regs
    iretq

.global iv_237
iv_237:
    call save_regs
    mov rdi, 237
    call isr_handler
    call restore_regs
    iretq

.global iv_238
iv_238:
    call save_regs
    mov rdi, 238
    call isr_handler
    call restore_regs
    iretq

.global iv_239
iv_239:
    call save_regs
    mov rdi, 239
    call isr_handler
    call restore_regs
    iretq

.global iv_240
iv_240:
    call save_regs
    mov rdi, 240
    call isr_handler
    call restore_regs
    iretq

.global iv_241
iv_241:
    call save_regs
    mov rdi, 241
    call isr_handler
    call restore_regs
    iretq

.global iv_242
iv_242:
    call save_regs
    mov rdi, 242
    call isr_handler
    call restore_regs
    iretq

.global iv_243
iv_243:
    call save_regs
    mov rdi, 243
    call isr_handler
    call restore_regs
    iretq

.global iv_244
iv_244:
    call save_regs
    mov rdi, 244
    call isr_handler
    call restore_regs
    iretq

.global iv_245
iv_245:
    call save_regs
    mov rdi, 245
    call isr_handler
    call restore_regs
    iretq

.global iv_246
iv_246:
    call save_regs
    mov rdi, 246
    call isr_handler
    call restore_regs
    iretq

.global iv_247
iv_247:
    call save_regs
    mov rdi, 247
    call isr_handler
    call restore_regs
    iretq

.global iv_248
iv_248:
    call save_regs
    mov rdi, 248
    call isr_handler
    call restore_regs
    iretq

.global iv_249
iv_249:
    call save_regs
    mov rdi, 249
    call isr_handler
    call restore_regs
    iretq

.global iv_250
iv_250:
    call save_regs
    mov rdi, 250
    call isr_handler
    call restore_regs
    iretq

.global iv_251
iv_251:
    call save_regs
    mov rdi, 251
    call isr_handler
    call restore_regs
    iretq

.global iv_252
iv_252:
    call save_regs
    mov rdi, 252
    call isr_handler
    call restore_regs
    iretq

.global iv_253
iv_253:
    call save_regs
    mov rdi, 253
    call isr_handler
    call restore_regs
    iretq

.global iv_254
iv_254:
    call save_regs
    mov rdi, 254
    call isr_handler
    call restore_regs
    iretq

.global iv_255
iv_255:
    call save_regs
    mov rdi, 255
    call isr_handler
    call restore_regs
    iretq


// end of handlers


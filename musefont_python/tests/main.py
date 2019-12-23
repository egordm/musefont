import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../libs/debug/'))
import musefont_python as mf

config = mf.FontConfig.load("/dv_drive/Development/Rust/muse_font/assets/fonts/smufl")
font = mf.ScoreFont.load("/dv_drive/Development/Rust/muse_font/assets/fonts/gootville", "gootville.otf", config)
id = config.get_symid("noteheadBlack")
pm = font.pixmap(id, mf.Size2F(1, 1), 64, mf.RasterizationOptions.GrayscaleAa, mf.Format.A8)


res = []
pixels = pm.pixels()
for y in range(0, pm.height()):
    for x in range(0, pm.width()):
        idx = x + y * pm.stride()
        if pixels[idx] > 0: res.append('#')
        else: res.append('.')
    res.append('\n')
print(''.join(res))
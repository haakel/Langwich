from PIL import Image, ImageDraw, ImageFont
import os

def create_icon():
    sizes = [16, 32, 48, 64, 128, 256]
    images = []
    
    for size in sizes:
        img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)
        
        margin = max(1, size // 16)
        radius = max(2, size // 4)
        
        # پس‌زمینه سرمه‌ای
        draw.rounded_rectangle(
            [margin, margin, size - margin - 1, size - margin - 1],
            radius=radius,
            fill=(44, 62, 80)
        )
        
        # حاشیه بنفش
        border = max(1, size // 32)
        draw.rounded_rectangle(
            [margin, margin, size - margin - 1, size - margin - 1],
            radius=radius,
            outline=(108, 92, 231),
            width=border
        )
        
        if size >= 32:
            # فونت
            font_size = max(8, size // 3)
            try:
                font = ImageFont.truetype("arial.ttf", font_size)
            except:
                font = ImageFont.load_default()
            
            # حرف «ف» بالا
            text_fa = "ف"
            bbox = draw.textbbox((0, 0), text_fa, font=font)
            tw = bbox[2] - bbox[0]
            draw.text(((size - tw) // 2, size // 6), text_fa, fill=(255, 255, 255), font=font)
            
            # خط جداکننده
            line_y = size // 2
            lm = size // 4
            lw = max(1, size // 32)
            draw.line([(lm, line_y), (size - lm, line_y)], fill=(108, 92, 231), width=lw)
            
            # «En» پایین
            font_en_size = max(6, size // 4)
            try:
                font_en = ImageFont.truetype("arial.ttf", font_en_size)
            except:
                font_en = ImageFont.load_default()
            bbox_en = draw.textbbox((0, 0), "En", font=font_en)
            tw_en = bbox_en[2] - bbox_en[0]
            draw.text(((size - tw_en) // 2, size // 2 + size // 8), "En", fill=(162, 155, 254), font=font_en)
        else:
            cx, cy = size // 2, size // 2
            r = max(1, size // 4)
            draw.ellipse([cx-r, cy-r, cx+r, cy+r], fill=(108, 92, 231))
        
        images.append(img)
    
    # ذخیره ico
    ico_path = os.path.join("F:", os.sep, "DevOver", "DevOver", "Resources", "Icons", "langwich.ico")
    os.makedirs(os.path.dirname(ico_path), exist_ok=True)
    images[-1].save(ico_path, format='ICO', sizes=[(s, s) for s in sizes], append_images=images[:-1])
    print(f"ICO saved: {ico_path}")
    
    # پیش‌نمایش PNG
    png_path = os.path.join("F:", os.sep, "DevOver", "langwich-icon-preview.png")
    images[-1].save(png_path)
    print(f"PNG saved: {png_path}")

create_icon()

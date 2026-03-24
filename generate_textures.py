from PIL import Image, ImageDraw

def create_grid_texture(filename, bg_color, line_color):
    size = 256
    img = Image.new('RGB', (size, size), color=bg_color)
    draw = ImageDraw.Draw(img)
    
    # Draw a simple grid
    step = 32
    for i in range(0, size, step):
        draw.line([(i, 0), (i, size)], fill=line_color, width=2)
        draw.line([(0, i), (size, i)], fill=line_color, width=2)
        
    # Draw a border
    draw.rectangle([(0, 0), (size-1, size-1)], outline=line_color, width=4)
    img.save(f"assets/{filename}")
    print(f"Generated assets/{filename}")

# Generate the requested textures
create_grid_texture("floor.png", (40, 40, 40), (100, 100, 100))        # Dark grey with light grids
create_grid_texture("pyramid.png", (200, 180, 50), (150, 100, 20))     # Sand/Gold
create_grid_texture("building.png", (50, 80, 120), (20, 40, 80))       # Steel blue
create_grid_texture("player_side.png", (180, 50, 50), (100, 20, 20))   # Red with dark red border
create_grid_texture("player_top.png", (50, 180, 50), (20, 100, 20))    # Green with dark green border

# Add this to the bottom of generate_textures.py and run it
def create_solid_texture(filename, color):
    img = Image.new('RGB', (16, 16), color=color)
    img.save(f"assets/{filename}")
    print(f"Generated assets/{filename}")

create_solid_texture("star.png", (255, 255, 255))


create_grid_texture("street_light.png", (60, 60, 60), (30, 30, 30)) # Dark metal
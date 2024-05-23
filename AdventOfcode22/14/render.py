import os
import imageio

path = os.path.dirname(os.path.abspath(__file__))
dir_list = os.listdir(path + "/images/")

def Load():
    imgs = []
    for file in dir_list:
        if file.endswith(".png"):
            imgs.append(file)
    with imageio.get_writer( path + '/movie.gif', mode='I') as writer:
        for filename in imgs:
            image = imageio.imread(path + "/images/"+ filename)
            writer.append_data(image)

Load()
gif = imageio.mimread( path + '/movie.gif', memtest=False)

imageio.mimsave( path + '/movie.gif', gif, fps=60)
print("end")
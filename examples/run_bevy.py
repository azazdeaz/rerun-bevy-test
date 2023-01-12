import rerun_bevy_test
import rerun
import asyncio
import numpy as np
import time


async def main():
    st = rerun_bevy_test.SimTest()
    st.run()
    images_sent = 0
    for _ in range(20):
        st.step()
        images= st.get_images()
        for image in images:
            (data, width, height) = image
            image = np.array(data).reshape((width, height, 4)).astype(np.uint8)
            rerun.log_image("world/camera/image/rgb", image)
            images_sent += 1
            print(f"Send image to rerun #{images_sent}")
            
    
if __name__ == "__main__":
    rerun.init("bevy")
    rerun.connect()
    asyncio.run(main())

import rerun_bevy_test
import rerun
import asyncio
import numpy as np


async def main():
    sim_test = rerun_bevy_test.SimTest()
    sim_test.run()
    images_sent = 0
    for _ in range(100):
        sim_test.step()
        images = sim_test.get_images()
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

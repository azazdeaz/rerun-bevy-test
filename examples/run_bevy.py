import rerun_bevy_test
import rerun
import asyncio
import numpy as np
from pynput import keyboard
import time


async def main():
    for _ in range(10):
        print("rerun_bevy_test", await rerun_bevy_test.get_image())
        rgb_image = np.random.randint(0, 255, (100, 200, 3)).astype(np.uint8)
        rerun.log_image("world/camera/image/rgb", rgb_image)
            
    
if __name__ == "__main__":
    rerun.init("bevy")
    rerun.connect()
    asyncio.run(main())

import rerun_bevy_test
import rerun
import asyncio
import numpy as np
from pynput import keyboard
import threading


async def main():
    st = rerun_bevy_test.SimTest()
    for _ in range(10):
        st.run()
        print("rerun_bevy_test", await rerun_bevy_test.get_image())
        rgb_image = np.random.randint(0, 255, (100, 200, 3)).astype(np.uint8)
        rerun.log_image("world/camera/image/rgb", rgb_image)
            
    
if __name__ == "__main__":
    rerun.init("bevy")
    rerun.connect()
    asyncio.run(main())

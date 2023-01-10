import rerun_bevy_test
import rerun

def main():
    print("rerun_bevy_test", rerun_bevy_test.test_images())
    # rerun.init("bevy")
    # rerun.log_image("world/camera/image/rgb", sample.rgb_image)
    # rerun.show()

if __name__ == "__main__":
    main()
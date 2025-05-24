from flask import Flask, render_template, Response
import cv2

app = Flask(__name__)


# Demo day issue:
# when a fpv is turned off, it crashes the flask server
# add exception handling


cap1 = cv2.VideoCapture(0) # Might need to change this to 1 depending on your camera / OS
cap2 = cv2.VideoCapture(1)
def gen_frames(video):  # generate frame by frame from camera
    while video.isOpened():
        # Capture frame-by-frame
        ret, frame = video.read()  # read the camera frame
        if ret:
            _, buffer = cv2.imencode('.jpg', frame)
            frame = buffer.tobytes()
            yield (b'--frame\r\n'
                   b'Content-Type: image/jpeg\r\n\r\n' + frame + b'\r\n')  # concat frame one by one and show result
        else:
            break
    video.release()
    cv2.destroyAllWindows()

@app.route('/mra_feed')
def video_feed():
    return Response(gen_frames(cap1), mimetype='multipart/x-mixed-replace; boundary=frame')

@app.route('/eru_feed')
def video_feed1():
    return Response(gen_frames(cap2), mimetype='multipart/x-mixed-replace; boundary=frame')


@app.route('/')
def index():
    """Video streaming home page."""
    return render_template('index.html')


if __name__ == '__main__':
    app.run(debug=True)

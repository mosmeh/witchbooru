import tensorflow as tf
import tflite
import tflite2onnx.model
import argparse


def main(args: argparse.Namespace):
    keras_model = tf.keras.models.load_model(args.model, compile=False)

    tflite_converter = tf.lite.TFLiteConverter.from_keras_model(keras_model)
    tflite_model = tflite_converter.convert()
    tflite_model = tflite.Model.GetRootAsModel(tflite_model, 0)

    onnx_model = tflite2onnx.model.Model(tflite_model)
    onnx_model.convert({})
    onnx_model.save(args.output)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('model', help='Model in Keras H5 format')
    parser.add_argument('-o', '--output')
    args = parser.parse_args()

    main(args)

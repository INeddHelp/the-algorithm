# pylint: disable=no-member, attribute-defined-outside-init, too-many-instance-attributes
"""
Implementing HashingDiscretizer Layer
"""

import libtwml
import tensorflow.compat.v1 as tf

import twml
from twml.constants import HashingDiscretizerOptions
from twml.layers.layer import Layer


class HashingDiscretizer(Layer):
    """A layer that discretizes continuous features, with hashed feature assignments

    HashingDiscretizer converts sparse continuous features into sparse
    binary features. Each binary output feature indicates the presence of a
    value in a HashingDiscretizer bin.

    Each calibrated HashingDiscretizer input feature is converted to n_bin+1 bins.

    - n_bin bin boundaries for each feature (i.e. len(bin_vals[id])==n_bin) defines n_bin+1 bins
    - bin assignment = sum(bin_vals<val)

    The difference between this layer and PercentileDiscretizer is that the
    HashingDiscretizer always assigns the same output id in the
    SparseTensor to the same input (feature id, bin) pair. This is useful if you
    want to user transfer learning on pre-trained sparse to dense embedding
    layers, but re-calibrate your discretizer on newer data.

    If there are no calibrated features, then the discretizer will only apply
    twml.util.limit_bits to the the feature keys (aka "feature_ids"). Essentially,
    the discretizer will be a "no-operation", other than obeying `out_bits`

    Typically, a HashingDiscretizer layer will be generated by calling the
    to_layer() method of the HashingDiscretizerCalibrator
    """

    def __init__(
        self,
        feature_ids,
        bin_vals,
        n_bin,
        out_bits,
        cost_per_unit=500,
        options=None,
        **kwargs
    ):
        """
        Creates a non-initialized `HashingDiscretizer` object.

        Parent class args:
          see [tf.layers.Layer](https://www.tensorflow.org/api_docs/python/tf/layers/Layer)
          for documentation of parent class arguments.

        Required args:
          feature_ids (1D int64 numpy array):
          - list of feature IDs that have been calibrated and have corresponding
            bin boundary values in the bin_vals array
          - bin values for feature feature_ids[i] live at bin_vals[i*n_bin:(i+1)*n_bin]
          bin_vals (1D float numpy array):
          - These are the bin boundary values for each calibrated feature
          - len(bin_vals) = n_bin*len(feature_ids)
          n_bin (int):
          - number of HashingDiscretizer bins is actually n_bin + 1
          - ***Note*** that if a value N is passed for the value of n_bin to
            HashingDiscretizerCalibrator, then HashingDiscretizerCalibrator
            will generate N+1 bin boundaries for each feature, and hence there
            will actually be N+2 potential bins for each feature
          out_bits (int):
            Determines the maximum value for output feature IDs.
            The dense_shape of the SparseTensor returned by lookup(x)
            will be [x.shape[0], 1 << output_bits].

        Optional args:
          cost_per_unit (int):
          - heuristic for intra op multithreading. approximate nanoseconds per input value.
          options (int or None for default):
          - Selects behavior of the op. Default is lower_bound and integer_multiplicative_hashing.
          - Use values in twml.constants.HashingDiscretizerOptions to select options as follows
            choose exactly one of HashingDiscretizerOptions.{SEARCH_LOWER_BOUND, SEARCH_LINEAR, SEARCH_UPPER_BOUND}
            choose exactly one of HashingDiscretizerOptions.{HASH_32BIT, HASH_64BIT}
            Bitwise OR these together to construct the options input.
            For example, `options=(HashingDiscretizerOptions.SEARCH_UPPER_BOUND | HashingDiscretizerOptions.HASH_64BIT)`
        """
        super(HashingDiscretizer, self).__init__(**kwargs)

        self._feature_ids = feature_ids
        self._bin_vals = bin_vals
        self._n_bin = n_bin
        self._out_bits = out_bits
        self.cost_per_unit = cost_per_unit
        if options is None:
            options = (
                HashingDiscretizerOptions.SEARCH_LOWER_BOUND
                | HashingDiscretizerOptions.HASH_32BIT
            )
        self._options = options

        if not self.built:
            self.build(input_shape=None)

    def build(self, input_shape):  # pylint: disable=unused-argument
        """
        Creates the variables of the layer
        """
        # make sure this is last
        self.built = True

    def call(self, inputs, **kwargs):
        """
        Implements HashingDiscretizer inference on a twml.SparseTensor.
        Alternatively, accepts a tf.SparseTensor that can be converted
        to twml.SparseTensor.

        Performs discretization of input values.
        i.e. bucket_val = bucket(val | feature_id)

        This bucket mapping depends on the calibration (i.e. the bin boundaries).
        However, (feature_id, bucket_val) pairs are mapped to new_feature_id in
        a way that is independent of the calibration procedure

        Args:
          inputs: A 2D SparseTensor that is input to HashingDiscretizer for
            discretization. It has a dense_shape of [batch_size, input_size]
          name: A name for the operation (optional).
        Returns:
          A tf.SparseTensor, created from twml.SparseTensor.to_tf()
          Its dense_shape is [shape_input.dense_shape[0], 1 << output_bits].
        """
        if isinstance(inputs, tf.SparseTensor):
            inputs = twml.SparseTensor.from_tf(inputs)

        if not (isinstance(inputs, twml.SparseTensor)):
            raise AssertionError

        # sparse column indices
        ids = inputs.ids
        # sparse row indices
        keys = inputs.indices
        # sparse values
        vals = inputs.values

        if len(self._feature_ids) > 0:
            # pass all inputs to the c++ op
            # the op determines whether to discretize (when a feature is calibrated),
            #   or whether to simply limit bits and pass through (when not calibrated)
            # NOTE - Hashing is done in C++
            discretizer_keys, discretizer_vals = libtwml.ops.hashing_discretizer(
                input_ids=keys,  # Input
                input_vals=vals,  # Input
                bin_vals=self._bin_vals,  # Input
                feature_ids=tf.make_tensor_proto(self._feature_ids),  # Attr
                n_bin=self._n_bin,  # Attr
                output_bits=self._out_bits,  # Attr
                cost_per_unit=self.cost_per_unit,  # Attr
                options=self._options,  # Attr
            )
        else:
            discretizer_keys = twml.util.limit_bits(keys, self._out_bits)
            discretizer_vals = vals

        batch_size = tf.to_int64(inputs.dense_shape[0])
        output_size = tf.convert_to_tensor(1 << self._out_bits, tf.int64)
        output_shape = [batch_size, output_size]

        return twml.SparseTensor(
            ids, discretizer_keys, discretizer_vals, output_shape
        ).to_tf()

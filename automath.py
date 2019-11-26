import tensorflow as tf
import tensorflow.keras.layers as layers

num_operators = 2
num_variable_inputs = 4
num_inputs = num_operators + num_variable_inputs
latent_dim = 150

def expression_loss(label, output):
    label = tf.cast(label, tf.int32)

    # split
    operators_o, variables_o = tf.split(output, [num_operators, num_variable_inputs], -1)
    operators_l, variables_l = tf.split(label, [1, 1], -1)
    operators_l = tf.squeeze(operators_l, [-1]) # [batch_size, timestep]
    variables_l = tf.squeeze(variables_l, [-1]) # [batch_size, timestep]

    operators_diff = tf.nn.softmax_cross_entropy_with_logits(tf.one_hot(operators_l, num_operators), operators_o) # [batch_size, timestep, num_operators]

    num_vars = tf.reduce_max(variables_l) # []
    mult_l = tf.tile(tf.stack([variables_l]), tf.stack([num_vars, 1, 1])) # [num_vars, batch_size, timestep]
    mask = tf.equal(tf.range(num_vars), mult_l) # [num_vars, batch_size, timestep]
    mult_o = tf.tile(tf.stack([variables_o]), tf.stack([num_vars, 1, 1, 1])) # [num_vars, batch_size, timestep, num_variable_inputs]
    masked = tf.ragged.boolean_mask(mult_o, mask) # [num_vars, batch_size, <timestep, num_variable_inputs]

    mean = tf.reduce_mean(masked, -2) # [num_vars, batch_size, num_variable_inputs]
    mean = mean.to_tensor()

    var1 = tf.math.reduce_variance(mean, 0) # [batch_size, num_variable_inputs]

    mean = tf.expand_dims(mean, -2) # [num_vars, batch_size, 1, num_variable_inputs]
    var2 = tf.reduce_mean(tf.square(masked - mean), -2) # [num_vars, batch_size, num_variable_inputs]

    loss = tf.reduce_sum(var2) + tf.reduce_sum(tf.square(operators_diff)) - tf.reduce_sum(var1)
    return loss

encoder_input = layers.Input(shape=(None, num_inputs))
encoder = layers.LSTM(latent_dim, activation="relu")(encoder_input)
decoder_input = layers.RepeatVector(tf.shape(encoder_input)[1])(encoder)
decoder = layers.LSTM(latent_dim, activation="relu", return_sequences=True)(decoder_input)
decoder_output = layers.TimeDistributed(layers.Dense(num_inputs))(decoder)

autoencoder = tf.keras.models.Model(encoder_input, decoder_output)
autoencoder.compile(optimizer="adam", loss=expression_loss)

# Embedding

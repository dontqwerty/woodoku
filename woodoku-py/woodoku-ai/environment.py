import os

import numpy as np

from tf_agents.environments import py_environment
from tf_agents.environments import utils
from tf_agents.specs import array_spec
from tf_agents.trajectories import time_step as ts

import woodoku_py

# Keep using keras-2 (tf-keras) rather than keras-3 (keras).
os.environ["TF_USE_LEGACY_KERAS"] = "1"


def number_to_base_with_resulting_digits(n, b, d):
    if n == 0:
        return [0] * d
    digits = []
    while n:
        digits.append(int(n % b))
        n //= b
    digits = digits[::-1]
    digits = [0] * (d - len(digits)) + digits
    return digits


class WoodokuEnv(py_environment.PyEnvironment):
    def __init__(self):
        self.w = woodoku_py.WoodokuPy()

        self.counter = 0

        board_size = self.w.board_size
        shapes_batch_size = self.w.shapes_batch_size
        shape_size = self.w.shape_size

        self._action_spec = array_spec.BoundedArraySpec(
            shape=(),
            dtype=np.int32,
            minimum=0,
            maximum=(board_size**shapes_batch_size) - 1,
        )
        self._observation_spec = array_spec.BoundedArraySpec(
            shape=(board_size + shapes_batch_size * shape_size,),
            dtype=np.int32,
            minimum=0,
            maximum=1,
        )

        self._state = self.get_state()
        self._episode_ended = False
        self._current_time_step = None

    def action_spec(self):
        return self._action_spec

    def observation_spec(self):
        return self._observation_spec

    def _reset(self):
        self.w = woodoku_py.WoodokuPy()
        self.counter = 0
        self._state = self.get_state()
        self._episode_ended = False
        self._current_time_step = None
        return ts.restart(self._state)

    def _step(self, action):
        self.counter += 1
        if self._episode_ended:
            # The last action ended the episode. Ignore the current action and start
            # a new episode.
            return self.reset()

        original_action = action.copy()
        decoded_action = number_to_base_with_resulting_digits(
            action, self.w.board_size, self.w.shapes_batch_size
        )
        print(f"{self.counter}: {original_action} -> {decoded_action}")
        reward = 0
        
        new_w = self.w
        exception_encountered = False
        for i in range(self.w.shapes_batch_size):
            try:
                new_w = new_w.play_move(i, decoded_action[i])
                reward += 1
            except Exception as e:
                print(f"{i}: {e}")
                exception_encountered = True
                reward -= 1
        if not exception_encountered:
            self.w = new_w

        if self.w.game_over:
            self._episode_ended = True
        else:
            self._state = self.get_state()

        if self._episode_ended:
            return ts.termination(self._state, reward=reward)
        else:
            return ts.transition(self._state, reward=reward, discount=1.0)

    def get_state(self):
        state = self.w.board
        # sum is used to flatten `shapes_batch`
        state.extend(sum(self.w.shapes_batch, []))

        return np.array(state, dtype=np.int32)


if __name__ == "__main__":
    env = WoodokuEnv()
    utils.validate_py_environment(env, episodes=1)

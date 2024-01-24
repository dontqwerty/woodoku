import os

import numpy as np

from tf_agents.environments import py_environment
from tf_agents.environments import utils
from tf_agents.specs import array_spec
from tf_agents.trajectories import time_step as ts

import woodoku_py

# Keep using keras-2 (tf-keras) rather than keras-3 (keras).
os.environ["TF_USE_LEGACY_KERAS"] = "1"


class WoodokuEnv(py_environment.PyEnvironment):
    def __init__(self):
        self.w = woodoku_py.WoodokuPy()

        board_size = self.w.board_size
        shapes_batch_size = self.w.shapes_batch_size
        shape_size = self.w.shape_size

        self._action_spec = array_spec.BoundedArraySpec(
            shape=(shapes_batch_size,),
            dtype=np.int32,
            minimum=0,
            maximum=board_size - 1,
        )
        self._observation_spec = array_spec.BoundedArraySpec(
            shape=(board_size + shapes_batch_size * shape_size,),
            dtype=np.int32,
            minimum=0,
            maximum=1,
        )

        self._state = self.get_state()
        self._episode_ended = False

    def action_spec(self):
        return self._action_spec

    def observation_spec(self):
        return self._observation_spec

    def _reset(self):
        self.w = woodoku_py.WoodokuPy()
        self._state = self.get_state()
        self._episode_ended = False
        return ts.restart(self._state)

    def _step(self, action):
        if self._episode_ended:
            # The last action ended the episode. Ignore the current action and start
            # a new episode.
            return self.reset()

        print(action)

        reward = 1
        for a_ix, a in enumerate(action):
            try:
                self.w = self.w.play_move(a_ix, a)
            except Exception:
                reward = 0

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
        state.extend(sum(self.w.shapes_batch, []))

        return np.array(state, dtype=np.int32)


if __name__ == "__main__":
    env = WoodokuEnv()
    utils.validate_py_environment(env, episodes=1)
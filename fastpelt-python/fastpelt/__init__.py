from .fastpeltrust import fit_predict
import numpy as np

class Pelt:
    def __init__(self, min_size=2, jump=5, loss="l2", pen=5):
        """
        Pruned Exact Linear Time

        Parameters
        ----------
        min_size : int
            Minimal size of partitions.
        jump : int
            Partition proposals at every `jump` t.
        loss : str
            "l1" or "l2"
        pen : float
            Penalty used in optimization.
        """
        self.min_size = min_size
        self.jump = jump
        self.loss = loss
        self.pen = pen

    def fit(self):
        """
        Does nothing
        """
        pass

    def predict(self, signal):
        """
        Parameters
        ----------
        signal : list/ np.array

        Returns
        -------
        changepoints : list

        """
        if not isinstance(signal, np.ndarray):
            signal = np.array(signal)

        length = len(signal.shape)
        if length == 2:
            signal = signal.T
        elif length == 1:
            signal = signal.reshape(1, -1)
        elif length > 2:
            raise ValueError("Wrong number of dimensions. Should be 2D.")       
            
        return fit_predict(
            signal, min_size=self.min_size, loss=self.loss, jump=self.jump, pen=self.pen
        )

    def fit_predict(self, signal):
        """
        Parameters
        ----------
        signal : np.array
            2D array. Columns are signals, rows are time dimension.

        Returns
        -------
        changepoints : list

        """
        return self.predict(signal)
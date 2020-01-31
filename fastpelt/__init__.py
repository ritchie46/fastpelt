from .fastpeltrust import fit_predict


class FastPelt:
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
        return fit_predict(
            signal, min_size=self.min_size, loss=self.loss, jump=self.jump, pen=self.pen
        )

    def fit_predict(self, signal):
        """
        Parameters
        ----------
        signal : list/ np.array

        Returns
        -------
        changepoints : list

        """
        return self.predict(signal)

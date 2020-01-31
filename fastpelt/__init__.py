from .fastpeltrust import fit_predict


class FastPelt:
    def __init__(self, min_size=2, jump=5, loss="l2", pen=5):
        self.min_size = min_size
        self.jump = jump
        self.loss = loss
        self.pen = pen

    def fit(self):
        pass

    def predict(self, signal):
        return fit_predict(
            signal, min_size=self.min_size, loss=self.loss, jump=self.jump, pen=self.pen
        )

    def fit_predict(self, signal):
        return self.predict(signal)

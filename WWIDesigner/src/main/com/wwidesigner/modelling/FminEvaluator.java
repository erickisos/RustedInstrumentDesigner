/**
 * Evaluate an instrument model based on deviation between measured and predicted minimum frequency.
 * <p>
 * Copyright (C) 2014, Edward Kort, Antoine Lefebvre, Burton Patkau.
 * <p>
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * <p>
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * <p>
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
package com.wwidesigner.modelling;

import java.util.List;

import com.wwidesigner.note.Fingering;
import com.wwidesigner.note.Note;
import com.wwidesigner.note.Tuning;

/**
 * Class to evaluate how well a calculator predicts fmin for an instrument,
 * based on deviation in cents.
 *
 * @author Burton Patkau
 */
public class FminEvaluator implements EvaluatorInterface {
    protected InstrumentCalculator calculator;
    protected InstrumentTuner tuner;

    public FminEvaluator(InstrumentCalculator aCalculator) {
        this.calculator = aCalculator;
        setTuner(new LinearVInstrumentTuner());
    }

    public FminEvaluator(InstrumentCalculator aCalculator, InstrumentTuner aTuner) {
        this.calculator = aCalculator;
        setTuner(aTuner);
    }

    /**
     * Return an array of cent differences between predicted and target fmin.
     *
     * @param fingeringActualData - Fingerings, with target note for each.
     * @return array of cent differences.   length = fingeringTargets.size().
     */

    @Override
    public double[] calculateErrorVector(List<Fingering> fingeringActualData) {
        Tuning targetTuning = new Tuning();
        targetTuning.setFingering(fingeringActualData);
        tuner.setTuning(targetTuning);

        return fingeringActualData
                .stream()
                .mapToDouble(actual -> actual.getNote() != null && actual.getNote().getFrequencyMin() != null
                        ? Note.cents(actual.getNote().getFrequencyMin(), tuner.predictedNote(actual).getFrequencyMin())
                        : 0.0)
                .toArray();
    }

    protected void setTuner(InstrumentTuner aTuner) {
        this.tuner = aTuner;
        this.tuner.setCalculator(calculator);
        this.tuner.setInstrument(calculator.getInstrument());
        this.tuner.setParams(calculator.getPhysicalParameters());
    }
}
/**
 * 
 */
package com.wwidesigner.optimization;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.fail;

import java.io.File;
import java.util.List;

import org.junit.Test;

import com.wwidesigner.geometry.BorePoint;
import com.wwidesigner.geometry.Instrument;
import com.wwidesigner.geometry.InstrumentConfigurator;
import com.wwidesigner.geometry.PositionInterface;
import com.wwidesigner.geometry.bind.GeometryBindFactory;
import com.wwidesigner.geometry.calculation.SimpleFippleMouthpieceConfigurator;
import com.wwidesigner.geometry.calculation.SimpleTestConfigurator;
import com.wwidesigner.note.Tuning;
import com.wwidesigner.note.bind.NoteBindFactory;
import com.wwidesigner.util.BindFactory;
import com.wwidesigner.util.Constants.TemperatureType;
import com.wwidesigner.util.PhysicalParameters;

/**
 * @author kort
 *
 */
public class NafOptimizationTest
{
	protected String inputInstrumentXML = "com/wwidesigner/optimization/example/NoHoleNAF1.xml";
	protected String inputTuningXML = "com/wwidesigner/optimization/example/NoHoleNAF1Tuning.xml";

	/**
	 * Complete workflow for optimizing an XML-defined instrument with the
	 * InstrumentOptimizer2 algorithm.
	 * 
	 * @return An Instrument object after optimization, with all dimensions in
	 *         the original units.
	 * @throws Exception
	 */
	public Instrument doInstrumentOptimization() throws Exception
	{
		Instrument instrument = getInstrumentFromXml(inputInstrumentXML);
		configureInstrument(instrument);

		Tuning tuning = getTuningFromXml(inputTuningXML);

		InstrumentOptimizer optimizer = new HolePositionOptimizer(
				instrument, tuning);
		setPhysicalParameters(optimizer);
		setOptimizationBounds(optimizer);
		optimizer.optimizeInstrument();

		// Convert back to the input unit-of-measure values
		instrument.convertToLengthType();

		// The optimizer modifies the input Instrument instance
		return instrument;
	}

	@Test
	public final void testInstrumentOptimization()
	{
		try
		{
			Instrument optimizedInstrument = doInstrumentOptimization();
			
			// Test bore length
			List<BorePoint> borePoints = optimizedInstrument.getBorePoint();
			PositionInterface[] sortedPoints = Instrument.sortList(borePoints);
			PositionInterface lastPoint = sortedPoints[sortedPoints.length - 1];
			assertEquals("Bore length incorrect", 12.2, lastPoint.getBorePosition(), 0.1);
			
		}
		catch (Exception e)
		{
			fail(e.getMessage());
		}
	}

	protected Instrument getInstrumentFromXml(String instrumentXML)
			throws Exception
	{
		BindFactory geometryBindFactory = GeometryBindFactory.getInstance();
		File inputFile = getInputFile(inputInstrumentXML, geometryBindFactory);
		Instrument instrument = (Instrument) geometryBindFactory.unmarshalXml(
				inputFile, true);

		return instrument;
	}

	protected void configureInstrument(Instrument instrument)
	{
		InstrumentConfigurator instrumentConfig = new SimpleFippleMouthpieceConfigurator();
		instrument.setConfiguration(instrumentConfig);

		// This unit-of-measure converter is called in setConfiguration(), but
		// is shown here to make it explicit. The method is efficient: it does
		// not redo the work.
		instrument.convertToMetres();
	}

	protected Tuning getTuningFromXml(String tuningXML) throws Exception
	{
		BindFactory noteBindFactory = NoteBindFactory.getInstance();
		File inputFile = getInputFile(inputTuningXML, noteBindFactory);
		Tuning tuning = (Tuning) noteBindFactory.unmarshalXml(inputFile, true);

		return tuning;
	}

	protected void setPhysicalParameters(InstrumentOptimizer optimizer)
	{
		PhysicalParameters parameters = new PhysicalParameters(22.22,
				TemperatureType.C);
		optimizer.setPhysicalParams(parameters);
	}

	protected void setOptimizationBounds(InstrumentOptimizer optimizer)
	{
		double[] lB = new double[1];
		double[] uB = new double[1];

		lB[0] = 0.25;
		uB[0] = 0.40;

		optimizer.setLowerBnd(lB);
		optimizer.setUpperBnd(uB);
	}

	/**
	 * This approach for get the input File is based on finding it in the
	 * classpath. The actual application will use an explicit file path - this
	 * approach will be unnecessary.
	 * 
	 * @param fileName
	 *            expressed as a package path.
	 * @param bindFactory
	 *            that manages the elements in the file.
	 * @return A file representation of the fileName, as found somewhere in the
	 *         classpath.
	 */
	protected File getInputFile(String fileName, BindFactory bindFactory)
	{
		String inputPath = bindFactory.getPathFromName(fileName);
		File inputFile = new File(inputPath);

		return inputFile;
	}
}

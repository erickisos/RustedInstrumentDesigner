//
// This file was generated by the JavaTM Architecture for XML Binding(JAXB) Reference Implementation, v2.2.5 
// See <a href="http://java.sun.com/xml/jaxb">http://java.sun.com/xml/jaxb</a> 
// Any modifications to this file will be lost upon recompilation of the source schema. 
// Generated on: 2012.06.30 at 09:14:21 PM MDT 
//


package com.wwidesigner.geometry.bind;

import javax.xml.bind.JAXBElement;
import javax.xml.bind.annotation.XmlElementDecl;
import javax.xml.bind.annotation.XmlRegistry;
import javax.xml.namespace.QName;


/**
 * This object contains factory methods for each 
 * Java content interface and Java element interface 
 * generated in the com.wwidesigner.geometry.bind package. 
 * <p>An ObjectFactory allows you to programatically 
 * construct new instances of the Java representation 
 * for XML content. The Java representation of XML 
 * content can consist of schema derived interfaces 
 * and classes representing the binding of schema 
 * type definitions, element declarations and model 
 * groups.  Factory methods for each of these are 
 * provided in this class.
 * 
 */
@XmlRegistry
public class ObjectFactory {

    private final static QName _Instrument_QNAME = new QName("http://www.wwidesigner.com/Instrument", "instrument");

    /**
     * Create a new ObjectFactory that can be used to create new instances of schema derived classes for package: com.wwidesigner.geometry.bind
     * 
     */
    public ObjectFactory() {
    }

    /**
     * Create an instance of {@link Mouthpiece }
     * 
     */
    public Mouthpiece createMouthpiece() {
        return new Mouthpiece();
    }

    /**
     * Create an instance of {@link Instrument }
     * 
     */
    public Instrument createInstrument() {
        return new Instrument();
    }

    /**
     * Create an instance of {@link Termination }
     * 
     */
    public Termination createTermination() {
        return new Termination();
    }

    /**
     * Create an instance of {@link ZeroOrMore }
     * 
     */
    public ZeroOrMore createZeroOrMore() {
        return new ZeroOrMore();
    }

    /**
     * Create an instance of {@link Key }
     * 
     */
    public Key createKey() {
        return new Key();
    }

    /**
     * Create an instance of {@link BorePoint }
     * 
     */
    public BorePoint createBorePoint() {
        return new BorePoint();
    }

    /**
     * Create an instance of {@link Dimension }
     * 
     */
    public Dimension createDimension() {
        return new Dimension();
    }

    /**
     * Create an instance of {@link Hole }
     * 
     */
    public Hole createHole() {
        return new Hole();
    }

    /**
     * Create an instance of {@link MoreThanZero }
     * 
     */
    public MoreThanZero createMoreThanZero() {
        return new MoreThanZero();
    }

    /**
     * Create an instance of {@link Mouthpiece.EmbouchureHole }
     * 
     */
    public Mouthpiece.EmbouchureHole createMouthpieceEmbouchureHole() {
        return new Mouthpiece.EmbouchureHole();
    }

    /**
     * Create an instance of {@link Mouthpiece.Fipple }
     * 
     */
    public Mouthpiece.Fipple createMouthpieceFipple() {
        return new Mouthpiece.Fipple();
    }

    /**
     * Create an instance of {@link JAXBElement }{@code <}{@link Instrument }{@code >}}
     * 
     */
    @XmlElementDecl(namespace = "http://www.wwidesigner.com/Instrument", name = "instrument")
    public JAXBElement<Instrument> createInstrument(Instrument value) {
        return new JAXBElement<Instrument>(_Instrument_QNAME, Instrument.class, null, value);
    }

}

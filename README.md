# Introduction

This is a simple command line utility decoding *ranges* property of PCI bridges device tree node. The interpretation of these cells is [here](http://www.devicetree.org/open-firmware/bindings/pci/pci2_1.pdf).

# Usage

```shell
$ pci-range 0x02000000 0x0 0x41000000  0x0 0x41000000  0x0 0x3f000000
PciAddress: MMIO32(0x41000000)
PhysicalAddress: 0x0000000041000000
Size: 0x3f000000
relocatable: false, prefetchable: false, aliased: false
```

The *ranges* property of PCI host bridge is grouped by seven cells.
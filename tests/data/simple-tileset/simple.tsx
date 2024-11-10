<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.11.0" name="simple" tilewidth="16" tileheight="16" tilecount="4" columns="0">
 <grid orientation="orthogonal" width="1" height="1"/>
 <tile id="0">
  <properties>
   <property name="speed" type="float" value="0.75"/>
   <property name="walkable" type="bool" value="true"/>
   <property name="water" type="bool" value="false"/>
  </properties>
  <image source="green.png" width="16" height="16"/>
 </tile>
 <tile id="1">
  <properties>
   <property name="speed" type="float" value="1"/>
   <property name="walkable" type="bool" value="true"/>
   <property name="water" type="bool" value="false"/>
  </properties>
  <image source="brown.png" width="16" height="16"/>
 </tile>
 <tile id="2">
  <properties>
   <property name="speed" type="float" value="0.5"/>
   <property name="walkable" type="bool" value="false"/>
   <property name="water" type="bool" value="true"/>
  </properties>
  <image source="blue.png" width="16" height="16"/>
 </tile>
 <tile id="3">
  <properties>
   <property name="speed" type="float" value="1"/>
   <property name="walkable" type="bool" value="false"/>
   <property name="water" type="bool" value="false"/>
  </properties>
  <image source="gray.png" width="16" height="16"/>
 </tile>
</tileset>

<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.4" tiledversion="1.4.2" name="default-tileset" tilewidth="16" tileheight="16" tilecount="2" columns="2">
 <editorsettings>
  <export target="default-tileset.json" format="json"/>
 </editorsettings>
 <properties>
  <property name="ron_path" value="textures/default_tileset.ron"/>
  <property name="texture_path" value="textures/default_tileset.png"/>
 </properties>
 <image source="default_tileset.png" trans="ff00ff" width="32" height="16"/>
 <tile id="0">
  <properties>
   <property name="collider" type="bool" value="true"/>
   <property name="gravity" type="bool" value="false"/>
  </properties>
 </tile>
 <tile id="1">
  <properties>
   <property name="collider" type="bool" value="false"/>
   <property name="gravity" type="bool" value="false"/>
  </properties>
 </tile>
</tileset>

<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>screen</name>
   <tag></tag>
   <elementGuidId>cd9c05c9-2be0-447b-a4ab-1994f3bea0b1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//html</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value></value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value></value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


online calcualtor

  	#animation_container 
  	{
		position:absolute;
		margin:auto;
		left:0;right:0;
		top:0;bottom:0;
 	}
  	*
	{
	  -moz-user-select: none;
	  -khtml-user-select: none;
	  -webkit-user-select: none;
	  user-select: none;
	}
	canvas 
	{
	  -webkit-tap-highlight-color: rgba(255, 255, 255, 0);
	}






var canvas, stage, exportRoot, anim_container, dom_overlay_container, fnStartAnimation, main;
function init() 
{
	canvas = document.getElementById(&quot;canvas&quot;);
	anim_container = document.getElementById(&quot;animation_container&quot;);
	dom_overlay_container = document.getElementById(&quot;dom_overlay_container&quot;);
	handleComplete();
}
function handleComplete() 
{
	exportRoot = new lib.vector();
	stage = new createjs.Stage(canvas);
	stage.addChild(exportRoot);
	createjs.Touch.enable(stage);
	stage.enableMouseOver();
	createjs.Touch.enable(stage);	

	fnStartAnimation = function() 
	{
		createjs.Ticker.setFPS(lib.properties.fps);
		createjs.Ticker.addEventListener(&quot;tick&quot;, stage);
	}	    

	function makeResponsive(isResp, respDim, isScale, scaleType) 
	{		
		var lastW, lastH, lastS=1;		
		window.addEventListener('resize', resizeCanvas);		
		resizeCanvas();		
		function resizeCanvas() 
		{			
			var w = lib.properties.width, h = lib.properties.height;			
			var iw = window.innerWidth, ih=window.innerHeight;			
			var pRatio = window.devicePixelRatio || 1, xRatio=iw/w, yRatio=ih/h, sRatio=1;			
			if(isResp) {                
				if((respDim=='width'&amp;&amp;lastW==iw) || (respDim=='height'&amp;&amp;lastH==ih)) 
				{                    
					sRatio = lastS;                
				}				
				else if(!isScale) 
				{					
					if(iw&lt;w || ih&lt;h)						
						sRatio = Math.min(xRatio, yRatio);				
				}				
				else if(scaleType==1) 
				{					
					sRatio = Math.min(xRatio, yRatio);				
				}				
				else if(scaleType==2) 
				{					
					sRatio = Math.max(xRatio, yRatio);				
				}			
			}			
			canvas.width = w*pRatio*sRatio;			
			canvas.height = h*pRatio*sRatio;
			canvas.style.width = dom_overlay_container.style.width = anim_container.style.width =  w*sRatio+'px';				
			canvas.style.height = anim_container.style.height = dom_overlay_container.style.height = h*sRatio+'px';
			stage.scaleX = pRatio*sRatio;			
			stage.scaleY = pRatio*sRatio;			
			lastW = iw; lastH = ih; lastS = sRatio;		
		}
	}
	makeResponsive(true,'both',true,1);	
	fnStartAnimation();
	main=new Main();
	document.addEventListener('keypress',iframeKeboardEventPress , false);
	document.addEventListener('keydown',iframeKeboardEvent,  false);
	
}
function iframeKeboardEvent(keyEvent)
{
	main.keyboardEventFromOutside(keyEvent);
}
function iframeKeboardEventDown(keyEvent)
{
	main.keyboardEventFromOutsideDown(keyEvent);
}
function iframeKeboardEventPress(keyEvent)
{
	
	main.keyboardEventFromOutsidePress(keyEvent);
}

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}







/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
